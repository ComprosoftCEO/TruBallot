use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use curv_kzen::BigInt;
use kzen_paillier::*;
use num::Zero;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::cmp::max;

use uuid_b64::UuidB64 as Uuid;

use crate::auth::{ClientToken, JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::{ClientRequestError, ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus, Registration};
use crate::notifications::{notify_registration_closed, notify_voting_opened};
use crate::protocol::{generator_prime_pair, stpm};
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
  //   Clear the access code, since it is no longer needed after registration closes
  election.status = ElectionStatus::InitFailed;
  election.access_code = None;
  election = election.update(&conn)?;
  notify_registration_closed(&election, &jwt_key).await;

  // We use a single prime that can serve the largest voting vector
  let questions_candidates = election.get_questions_candidates_ordered(&conn)?;
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

  // Generate the STPM Paillier cryptosystem key pair
  // Should have enough bits to store our locations without any modulus
  let num_bits = 512;
  log::debug!("Generate Paillier keypair with {} bits", num_bits);
  let (encryption_key, decryption_key) = Paillier::keypair_safe_primes_with_modulus_size(512).keys();

  // Encrypt the locations 0 to N using STPM (Step 1)
  log::debug!("Encrypting locations 0 .. {} using STPM", registrations.len() - 1);
  let mut encrypted_locations: Vec<BigInt> = (0u64..(registrations.len() as u64))
    .into_iter()
    .map(|i| stpm::step_1(&BigInt::from(i), &encryption_key.n))
    .collect();

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
    encrypted_locations,
    n: Some(encryption_key.n.clone()),
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
  let mut encrypted_locations = collector1_response.encryption_result;
  if encrypted_locations.len() != registrations.len() {
    return Err(ServiceError::WrongNumberOfEncryptedLocations {
      collector: Collector::Two,
      given: encrypted_locations.len(),
      expected: registrations.len(),
    });
  }

  // Perform step 3 of STPM to get the encrypted locations for collector 2
  log::debug!("Decrypt locations using STPM to get r1");
  encrypted_locations
    .iter_mut()
    .for_each(|l| *l = stpm::step_3(l, &decryption_key.p, &decryption_key.q, true));

  // Register the election with the second collector
  log::debug!("Send election parameters to collector 2");
  create_elections_data.encrypted_locations = encrypted_locations;
  create_elections_data.n = None;
  let collector2_request = Client::builder()
    .disable_timeout()
    .bearer_auth(ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
    .finish()
    .post(Collector::Two.api_url("/elections")?)
    .send_json(&create_elections_data);

  let _: CreateElectionResponse = ClientRequestError::handle(collector2_request)
    .await
    .map_err(|e| ServiceError::RegisterElectionError(Collector::Two, e))?;
  log::debug!("Got success response from collector 2");

  // Note:
  //   Collector 2 won't return any encryption result, so we don't need to check here...
  //   Both collectors now have r1 + r2 = Encrypted location

  // Election is now FULLY INITIALIZED!!!
  log::debug!("Marking election as fully initialized...");
  election.status = ElectionStatus::Voting;
  election.update(&conn)?;

  notify_voting_opened(&election, &jwt_key).await;
  log::info!(
    "Voting initialized for election \"{}\" <{}>",
    election.name,
    election.id
  );

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

  /// Use secure two-party multiplication to store location
  /// If n is provided, then this is on step 2, otherwise it is on step 3
  #[serde(with = "kzen_paillier::serialize::vecbigint")]
  encrypted_locations: Vec<BigInt>,
  #[serde(with = "crate::utils::serialize_option_bigint")]
  n: Option<BigInt>,
}

#[derive(Debug, Serialize)]
struct CreateElectionQuestion {
  id: Uuid,
  num_candidates: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateElectionResponse {
  // Vector might be empty when returning from the second collector
  #[serde(with = "kzen_paillier::serialize::vecbigint")]
  encryption_result: Vec<BigInt>,
}
