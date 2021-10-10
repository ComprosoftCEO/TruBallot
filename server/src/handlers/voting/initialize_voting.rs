use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, NewBlockCipher};
use aes::{Aes256, Block, BLOCK_SIZE};
use curv_kzen::BigInt;
use diesel::prelude::*;
use num::Zero;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::cmp::max;

use uuid_b64::UuidB64 as Uuid;

use crate::auth::{ClientToken, JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::{ClientRequestError, ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus, Registration};
use crate::protocol::generator_prime_pair;
use crate::utils::ConvertBigInt;
use crate::Collector;

pub async fn initialize_voting(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;

  // Make sure the election exists
  let mut election = Election::find_resource(&*path, &conn)?;

  // Only the election creator can open the election for voting
  let current_user_id = token.get_user_id();
  if election.created_by != current_user_id {
    return Err(ServiceError::ElectionNotOwnedByUser {
      current_user_id,
      owner_id: election.created_by,
      action: ResourceAction::InitVoting,
    });
  }

  // Make sure the election is in the correct status
  if !(election.status == ElectionStatus::Registration || election.status == ElectionStatus::InitFailed) {
    return Err(ServiceError::WrongStatusFor {
      election_id: election.id,
      action: ResourceAction::InitVoting,
      status: election.status,
    });
  }

  // Election MUST have at least 4 users registered
  let registrations: Vec<Registration> = election.get_registrations(&conn)?;
  if registrations.len() < 4 {
    return Err(ServiceError::NotEnoughRegistered {
      election_id: election.id,
      num_registered: registrations.len(),
    });
  }

  // Mark the election as being initialized
  election.status = ElectionStatus::InitFailed;
  election = election.update(&conn)?;

  // We use a single prime that can serve the largest voting vector
  let questions_candidates = election.get_questions_candidates(&conn)?;
  let max_num_candidates = questions_candidates.iter().map(|(_, c)| c.len()).max().unwrap_or(2);
  let voting_vector_max_bits = registrations.len() * max_num_candidates;

  // Since we may call this method multiple times, only generate if we haven't done so before
  if election.generator.is_zero() || election.prime.is_zero() {
    let num_bits = max(2 * voting_vector_max_bits + 1, 256);
    log::debug!("Generating prime with {} bits", num_bits);

    let (generator, prime) = generator_prime_pair(num_bits);
    election.generator = generator.to_bigdecimal();
    election.prime = prime.to_bigdecimal();
    election = election.update(&conn)?;

    log::debug!("Picked g = {} and p = {}", generator, prime);
  }

  // Encrypt the locations 0 to N using AES
  let key = GenericArray::from_slice(election.encryption_key.as_slice());
  let cipher = Aes256::new(&key);

  log::debug!("Encrypting locations 0 .. {}", registrations.len());
  let mut encrypted_locations: Vec<Block> = (0u128..(registrations.len() as u128))
    .into_iter()
    .map(|i| Block::from(i.to_be_bytes()))
    .collect();
  cipher.encrypt_blocks(&mut encrypted_locations);

  // Then shuffle the list
  log::debug!("Shuffle encrypted locations");
  encrypted_locations.shuffle(&mut thread_rng());

  // Build data needed to register the election with both collectors
  let jwt_encoding_key = jwt_key.get_encoding_key();
  let mut create_elections_data = CreateElectionData {
    id: election.id,
    generator: election.generator.to_bigint(),
    prime: election.prime.to_bigint(),
    questions: questions_candidates
      .iter()
      .map(|(question, candidates)| CreateElectionQuestion {
        id: question.id,
        num_candidates: candidates.len() as i64,
      })
      .collect(),
    registered_users: registrations.iter().map(|r| r.user_id).collect(),
    encrypted_locations: encrypted_locations.into_iter().map(|e| *e.as_ref()).collect(),
  };

  // Register the election with the first collector
  log::debug!("Send election parameters to collector 1");
  let collector1_request = Client::builder()
    .disable_timeout()
    .bearer_auth(ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
    .finish()
    .post(Collector::One.api_url("/elections")?)
    .send_json(&create_elections_data);

  let collector1_response: CreateElectionResponse = ClientRequestError::handle(collector1_request)
    .await
    .map_err(|e| ServiceError::RegisterElectionError(Collector::One, e))?;
  log::debug!("Got success response from collector 1");

  // Make sure collector 1 returned the correct number of encrypted locations
  let encrypted_locations = collector1_response.encrypted_locations;
  if encrypted_locations.len() != registrations.len() {
    return Err(ServiceError::WrongNumberOfEncryptedLocations {
      collector: Collector::Two,
      given: encrypted_locations.len(),
      expected: registrations.len(),
    });
  }

  // Register the election with the second collector
  log::debug!("Send election parameters to collector 2");
  create_elections_data.encrypted_locations = encrypted_locations;
  let collector2_request = Client::builder()
    .disable_timeout()
    .bearer_auth(ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
    .finish()
    .post(Collector::Two.api_url("/elections")?)
    .send_json(&create_elections_data);

  let collector2_response: CreateElectionResponse = ClientRequestError::handle(collector2_request)
    .await
    .map_err(|e| ServiceError::RegisterElectionError(Collector::Two, e))?;
  log::debug!("Got success response from collector 2");

  // Make sure collector 2 returned the correct number of encrypted locations
  let encrypted_locations = collector2_response.encrypted_locations;
  if encrypted_locations.len() != registrations.len() {
    return Err(ServiceError::WrongNumberOfEncryptedLocations {
      collector: Collector::Two,
      given: encrypted_locations.len(),
      expected: registrations.len(),
    });
  }

  // Give an encrypted location to each registered user
  log::debug!("Giving an encrypted location to each registered user");
  conn.get().transaction::<(), ServiceError, _>(|| {
    for (mut registration, location) in registrations.into_iter().zip(encrypted_locations.into_iter()) {
      registration.encrypted_location = location.to_vec();
      registration.update(&conn)?;
    }

    // Election is now FULLY INITIALIZED!!!
    election.status = ElectionStatus::Voting;
    election.update(&conn)?;

    Ok(())
  })?;

  Ok(HttpResponse::Ok().finish())
}

///
/// JSON structure to send to the collectors to register an election
///
#[derive(Debug, Serialize)]
struct CreateElectionData {
  id: Uuid,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  generator: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  prime: BigInt,

  questions: Vec<CreateElectionQuestion>,
  registered_users: Vec<Uuid>,
  encrypted_locations: Vec<[u8; BLOCK_SIZE]>,
}

#[derive(Debug, Serialize)]
struct CreateElectionQuestion {
  id: Uuid,
  num_candidates: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateElectionResponse {
  encrypted_locations: Vec<[u8; BLOCK_SIZE]>,
}
