use actix_web::{web, HttpResponse};
use curv_kzen::{arithmetic::BitManipulation, BigInt};
use diesel::prelude::*;
use kzen_paillier::*;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::{Validate, ValidationError};

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{HasPermission, MediatorToken};
use crate::models::{Election, EncryptedLocation, Question, Registration};
use crate::protocol::{location_anonymization as loc_anon, SharesMatrix};
use crate::utils::ConvertBigInt;
use crate::views::election::CreateElectionResponse;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_struct", skip_on_field_errors = false))]
#[serde(rename_all = "camelCase")]
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

  #[validate(range(min = 2))]
  num_collectors: usize,
  collector_index: usize,

  /// Use secure two-party multiplication to store location
  /// If n is provided, then this is on step 2, otherwise it is on step 3
  #[serde(with = "kzen_paillier::serialize::vecbigint")]
  encrypted_locations: Vec<BigInt>,
  #[serde(with = "crate::utils::serialize_option_bigint")]
  n: Option<BigInt>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionQuestion {
  id: Uuid,

  #[validate(range(min = 2))]
  num_candidates: i64,
}

///
/// Special validator function to make sure the following conditions hold:
///   1. The collector index is within the range for the number of collectors
///   2. There are enough users for the election
///   3. There is an encrypted location entry for every registered user
///
fn validate_struct(input: &CreateElectionData) -> Result<(), ValidationError> {
  if input.collector_index >= input.num_collectors {
    return Err(ValidationError::new(
      "collector_index is greater than or equal to num_collectors",
    ));
  }

  if input.registered_users.len() < 2 * input.num_collectors {
    return Err(ValidationError::new(
      "must be at least 2*num_collectors users to be registered in the election",
    ));
  }

  if input.registered_users.len() != input.encrypted_locations.len() {
    return Err(ValidationError::new(
      "length(registered_users) not equal to length(encrypted_locations)",
    ));
  }

  Ok(())
}

pub async fn create_and_initialize_election(
  token: MediatorToken,
  data: web::Json<CreateElectionData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  data.validate()?;

  let mut data = data.into_inner();

  // Handle the encrypted anonymous locations protocol
  log::debug!(
    "Computing value r{} for encrypted locations",
    ((data.collector_index + 1) % data.num_collectors) + 1
  );
  let (encrypted_locations, encryption_result) = handle_encrypted_location(
    data.encrypted_locations.drain(..).collect(),
    &data.n,
    data.collector_index == 0,
  );

  // Create the election if it does not already exist
  //  Otherwise, we update the locations on the existing election
  if let Some(election) = Election::find_optional(&data.id, &conn)? {
    update_encrypted_locations(&data, &election, &encrypted_locations, &conn)?;
  } else {
    create_new_election(&data, &encrypted_locations, &conn)?;
  }

  // Done!
  Ok(HttpResponse::Ok().json(CreateElectionResponse { encryption_result }))
}

/// Handle the encrypted anonymous location protocol
///
/// If n is provided, then we are at step i of location anonymization.
/// Otherwise, we are at the last step and don't need to do any more encryption.
///
/// The "should_shuffle" flag should ONLY be set for the first collector
///  in the list (index = 0) for the protocol to work correctly
///
/// Returns: (r_i, results)
///   -"results" is an empty vector if we are at step 3
fn handle_encrypted_location(
  encrypted_locations: Vec<BigInt>,
  n: &Option<BigInt>,
  should_shuffle: bool,
) -> (Vec<BigInt>, Vec<BigInt>) {
  if let Some(ref n) = n {
    // Perform step i on all locations
    let mut encrypted_locations: Vec<_> = encrypted_locations
      .into_iter()
      .map(|l| loc_anon::step_ith(&l, n))
      .collect();

    // Shuffle the order of locations for the first collector being initialized
    if should_shuffle {
      log::debug!("Shuffling the list of encrypted locations");
      encrypted_locations.shuffle(&mut thread_rng());
    }

    encrypted_locations.into_iter().unzip()
  } else {
    // We are at step 3, do nothing
    (encrypted_locations, Vec::new())
  }
}

///
/// Create a new election since one does not currently exist in the database.
///
/// This initializes all shares and parameters within the collector.
/// It also initializes the user locations.
///
fn create_new_election(
  data: &CreateElectionData,
  encrypted_locations: &[BigInt],
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
    let _questions = data
      .questions
      .iter()
      .map(|question| Question::new(question.id, election.id, question.num_candidates).insert(&conn))
      .collect::<Result<Vec<Question>, _>>()?;

    // Create all of the encrypted locations
    data
      .registered_users
      .iter()
      .zip(encrypted_locations.iter())
      .map(|(user_id, location)| EncryptedLocation::new(*user_id, election.id, location).insert(&conn))
      .collect::<Result<Vec<_>, _>>()?;

    // Finally, create all of the registrations with the user shares
    generate_question_n_n_secret_shares(data, conn)?;

    Ok(election)
  })?)
}

///
/// Update the encrypted locations with the new values
///
fn update_encrypted_locations(
  data: &CreateElectionData,
  election: &Election,
  encrypted_locations: &[BigInt],
  conn: &DbConnection,
) -> Result<(), ServiceError> {
  conn.get().transaction::<_, ServiceError, _>(|| {
    data
      .registered_users
      .iter()
      .zip(encrypted_locations.iter())
      .map(|(user_id, encrypted_location)| {
        // Update each individual encrypted locations
        let mut location = EncryptedLocation::find_resource(user_id, &election.id, conn)?;
        location.location = encrypted_location.to_bigdecimal();
        Ok(location.update(conn)?)
      })
      .collect::<Result<Vec<_>, ServiceError>>()?;

    // Also regenerate all of the user shares, as the collector order might have changed
    generate_question_n_n_secret_shares(data, conn)?;

    Ok(())
  })
}

///
/// Generate all of the shares for (n,n)-Secret Sharing
///
/// This method should be called inside the database transaction to keep everything atomic.
///
fn generate_question_n_n_secret_shares(data: &CreateElectionData, conn: &DbConnection) -> Result<(), ServiceError> {
  // We create a separate table for each question
  for (question, question_number) in data.questions.iter().zip(1usize..) {
    // Clear the existing list of user registrations (We will replace them)
    Question::delete_all_registrations_from_id(&question.id, &conn)?;

    // Generate all of the shares for the voters for each question
    //  Since our generator g^x (mod p) is order p - 1 (NOT order p), our shares are mod (p-1)
    log::debug!("Generate shares for question {}", question_number);
    let forward_shares = SharesMatrix::new(
      data.collector_index,
      data.num_collectors,
      data.registered_users.len(),
      &data.prime - 1,
    );

    let reverse_shares = SharesMatrix::new(
      data.collector_index,
      data.num_collectors,
      data.registered_users.len(),
      &data.prime - 1,
    );

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
        election_id: data.id,
        question_id: question.id,
        forward_verification_shares,
        reverse_verification_shares,
        forward_ballot_shares,
        reverse_ballot_shares,
      }
      .insert(conn)?;
    }
  }

  Ok(())
}
