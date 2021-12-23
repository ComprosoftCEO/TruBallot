use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use curv_kzen::BigInt;
use num::Zero;
use serde::{Deserialize, Serialize};
use std::cmp::max;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::auth::{ClientToken, JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::config;
use crate::db::DbConnection;
use crate::errors::{ClientRequestError, ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus, Registration};
use crate::notifications::{notify_registration_closed, notify_voting_opened};
use crate::protocol::generator_prime_pair;
use crate::utils::ConvertBigInt;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InitializeVotingData {
  #[validate(length(min = 2))]
  collectors: Vec<Uuid>,
}

pub async fn initialize_voting(
  token: ClientToken,
  path: web::Path<Uuid>,
  data: web::Json<InitializeVotingData>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;
  data.validate()?;

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

  // Election MUST have at least 2*(num collectors) users registered
  let registrations: Vec<Registration> = election.get_registrations(&conn)?;
  if registrations.len() < 2 * data.collectors.len() {
    return Err(ServiceError::NotEnoughRegistered {
      election_id: election.id,
      num_registered: registrations.len(),
      num_collectors: data.collectors.len(),
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

  // Build data needed to register the election with the mediator
  let jwt_encoding_key = jwt_key.get_encoding_key();
  let create_elections_data = CreateElectionData {
    id: election.id,
    is_public: election.is_public,
    creator_id: election.created_by,
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
    collectors: data.into_inner().collectors,
  };

  // Build the URL to the mediator API
  let mediator_url = config::get_mediator_url().ok_or_else(|| ServiceError::MediatorURLNotSet)?;
  let url = format!("{}/api/v1/mediator/elections", mediator_url);

  // Register the election with the collector mediator
  log::debug!("Send election parameters to collector mediator");
  let mediator_request = Client::builder()
    .disable_timeout()
    .bearer_auth(ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
    .finish()
    .post(&url)
    .send_json(&create_elections_data);

  let result: InitializeElectionResult = ClientRequestError::handle(mediator_request)
    .await
    .map_err(|e| ServiceError::RegisterElectionError(e))?;

  // Election is now FULLY INITIALIZED!!!
  log::debug!("Got success response from mediator");
  log::debug!("Marking election as fully initialized...");
  election.location_modulus = result.n.to_bigdecimal();
  election.status = ElectionStatus::Voting;
  election.update(&conn)?;

  notify_voting_opened(&election, create_elections_data.collectors, &jwt_key).await;
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
#[serde(rename_all = "camelCase")]
struct CreateElectionData {
  id: Uuid,
  is_public: bool,
  creator_id: Uuid,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  generator: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  prime: BigInt,

  questions: Vec<CreateElectionQuestion>,
  registered_users: Vec<Uuid>,
  collectors: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateElectionQuestion {
  id: Uuid,
  num_candidates: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializeElectionResult {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  n: BigInt,
}
