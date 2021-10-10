use actix_web::{web, HttpResponse};
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, NewBlockCipher};
use aes::{Aes256, Block, BLOCK_SIZE};
use curv_kzen::{arithmetic::BitManipulation, BigInt};
use diesel::prelude::*;
use kzen_paillier::*;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::{Validate, ValidationError};

use crate::auth::ServerToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::{Election, Question, Registration};
use crate::protocol::SharesMatrix;
use crate::utils::ConvertBigInt;
use crate::views::election::CreateElectionResponse;
use crate::Collector;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_encrypted_locations", skip_on_field_errors = false))]
pub struct CreateElectionData {
  id: Uuid,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  generator: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  prime: BigInt,

  #[validate(length(min = 1))]
  #[validate]
  questions: Vec<CreateElectionQuestion>,

  #[validate(length(min = 2))]
  registered_users: Vec<Uuid>,

  encrypted_locations: Vec<[u8; BLOCK_SIZE]>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateElectionQuestion {
  id: Uuid,

  #[validate(range(min = 2))]
  num_candidates: i64,
}

///
/// Special validator function to make sure there is an encrypted location entry for every registered user
///
fn validate_encrypted_locations(input: &CreateElectionData) -> Result<(), ValidationError> {
  if input.registered_users.len() != input.encrypted_locations.len() {
    return Err(ValidationError::new(
      "length(registered_users) not equal to length(encrypted_locations)",
    ));
  }

  Ok(())
}

pub async fn create_and_initialize_election(
  token: ServerToken,
  data: web::Json<CreateElectionData>,
  collector: web::Data<Collector>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  data.validate()?;

  let data = data.into_inner();

  // Create the election if it does not already exist
  //  Otherwise, we don't do anything...
  let election = match Election::find_optional(&data.id, &conn)? {
    Some(election) => election,
    None => create_new_election(&data, *collector.get_ref(), &conn)?,
  };

  // Perform the encryption
  let key = GenericArray::from_slice(election.encryption_key.as_slice());
  let cipher = Aes256::new(&key);
  let mut encrypted_locations: Vec<_> = data.encrypted_locations.into_iter().map(Block::from).collect();
  cipher.encrypt_blocks(&mut encrypted_locations);

  // And shuffle the blocks
  encrypted_locations.shuffle(&mut thread_rng());

  // Done!
  Ok(HttpResponse::Ok().json(CreateElectionResponse {
    encrypted_locations: encrypted_locations.into_iter().map(|p| *p.as_ref()).collect(),
  }))
}

///
/// Create a new election since one does not currently exist in the database
/// This initializes all shares and parameters within the collector
///
fn create_new_election(
  data: &CreateElectionData,
  collector: Collector,
  conn: &DbConnection,
) -> Result<Election, ServiceError> {
  // Generate the STPM Paillier cryptosystem key pair
  // Should have enough bits to store x1 * x2 without any modulus
  let num_bits = 4 * data.prime.bit_length();
  log::debug!("Generate Paillier keypair with {} bits", num_bits);
  let (_, decryption_key) = Paillier::keypair_safe_primes_with_modulus_size(num_bits).keys();

  Ok(conn.get().transaction::<Election, ServiceError, _>(|| {
    // Create the election
    let election = Election::new(data.id, &data.generator, &data.prime, &decryption_key).insert(&conn)?;

    // Create the questions for the election
    let questions = data
      .questions
      .iter()
      .map(|question| Question::new(question.id, election.id, question.num_candidates).insert(&conn))
      .collect::<Result<Vec<Question>, _>>()?;

    // Finally, create all of the registrations with the user shares
    for (question, question_number) in questions.into_iter().zip(1usize..) {
      // Generate all of the shares for the voters for each question
      //  Since our generator g^x (mod p) is order p - 1 (NOT order p), our shares are mod (p-1)
      log::debug!("Generate shares for question {}", question_number);
      let forward_shares = SharesMatrix::new(collector, data.registered_users.len(), &data.prime - 1);
      let reverse_shares = SharesMatrix::new(collector, data.registered_users.len(), &data.prime - 1);

      // Now register all of the users!
      log::debug!("Register users for question {}", question_number);
      for (user_index, user_id) in data.registered_users.iter().enumerate() {
        // S_c,i
        let forward_verification_shares = forward_shares
          .get_verification_shares(user_index)
          .unwrap()
          .to_bigdecimal();

        // S_c,i'
        let reverse_verification_shares = reverse_shares
          .get_verification_shares(user_index)
          .unwrap()
          .to_bigdecimal();

        // S~c,i
        let forward_ballot_shares = forward_shares.get_ballot_shares(user_index).unwrap().to_bigdecimal();

        // S~c,i'
        let reverse_ballot_shares = reverse_shares.get_ballot_shares(user_index).unwrap().to_bigdecimal();

        Registration {
          user_id: *user_id,
          election_id: election.id,
          question_id: question.id,
          forward_verification_shares,
          reverse_verification_shares,
          forward_ballot_shares,
          reverse_ballot_shares,
        }
        .insert(conn)?;
      }
    }

    Ok(election)
  })?)
}
