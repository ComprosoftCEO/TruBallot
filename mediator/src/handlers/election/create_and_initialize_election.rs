use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use curv_kzen::BigInt;
use diesel::prelude::*;
use kzen_paillier::*;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::auth::{JWTSecret, MediatorToken, ServerToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::{ClientRequestError, ServiceError};
use crate::models::{Collector, Election, ElectionCollector, Question, Registration};
use crate::protocol::location_anonymization as loc_anon;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionData {
  id: Uuid,
  is_public: bool,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  generator: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  prime: BigInt,

  #[validate(length(min = 1))]
  #[validate]
  questions: Vec<CreateElectionQuestion>,

  #[validate(length(min = 2))]
  registered_users: Vec<Uuid>,

  #[validate(length(min = 2))]
  collectors: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionQuestion {
  id: Uuid,

  #[validate(range(min = 2))]
  num_candidates: i64,
}

pub async fn create_and_initialize_election(
  token: ServerToken,
  data: web::Json<CreateElectionData>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  data.validate()?;

  let data = data.into_inner();

  // Make sure we have enough registered users based on the number of collectors
  if data.registered_users.len() < 2 * data.collectors.len() {
    return Err(ServiceError::NotEnoughUsers {
      given: data.registered_users.len(),
      expected: 2 * data.collectors.len(),
    });
  }

  // See if the election already exists in the database
  //  (This means it has already been created and initialized, so do nothing)
  if Election::exists_from_id(&data.id, &conn)? {
    return Ok(HttpResponse::Ok().finish());
  }

  // Get all collectors from the database
  let collectors: Vec<Collector> = data
    .collectors
    .iter()
    .map(|collector_id| Collector::find_resource(collector_id, &conn))
    .collect::<Result<_, _>>()?;

  // Generate the STPM Paillier cryptosystem key pair
  // Should have enough bits to store our locations without any modulus
  let num_bits = 512;
  log::debug!("Generate Paillier keypair with {} bits", num_bits);
  let (encryption_key, decryption_key) = Paillier::keypair_safe_primes_with_modulus_size(num_bits).keys();

  // Encrypt the locations 0 to N-1 using the location anonymization protocol
  log::debug!(
    "Encrypting locations 0 .. {} for location anonymization",
    data.registered_users.len() - 1
  );
  let mut encrypted_locations: Vec<BigInt> = (0u64..(data.registered_users.len() as u64))
    .into_iter()
    .map(|i| loc_anon::step_1(&BigInt::from(i), &encryption_key.n))
    .collect();

  // Then shuffle the list
  log::debug!("Shuffle encrypted locations");
  encrypted_locations.shuffle(&mut thread_rng());

  // Build data needed to register the election with both collectors
  //   Most of the data is the same, but some of values need to be updated per collector
  let jwt_encoding_key = jwt_key.get_encoding_key();
  let mut create_elections_data = CollectorCreateElectionData {
    id: data.id,
    is_public: data.is_public,
    generator: data.generator,
    prime: data.prime,
    questions: data.questions,
    registered_users: data.registered_users,
    num_collectors: collectors.len(),
    collector_index: 0,
    encrypted_locations,
    n: Some(encryption_key.n.clone()),
  };

  // =========================================
  //   Talk with each collector in order
  // =========================================
  for (index, collector) in collectors.iter().enumerate() {
    // Set the index for the current collector
    create_elections_data.collector_index = index;

    // If we are on the last collector, decrypt the locations for the protocol
    if index == collectors.len() - 1 {
      create_elections_data.n = None;

      log::debug!("Decrypt locations for the final collector");
      create_elections_data
        .encrypted_locations
        .iter_mut()
        .for_each(|l| *l = loc_anon::step_last(l, &decryption_key.p, &decryption_key.q));
    }

    // Build the request object for the collector
    log::debug!(
      "Send election parameters to collector {} of {}",
      index + 1,
      collectors.len()
    );
    let collector_request = Client::builder()
      .disable_timeout()
      .bearer_auth(MediatorToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
      .finish()
      .post(collector.private_api_url("/elections"))
      .send_json(&create_elections_data);

    // Send the request and handle any errors
    let collector_response: CreateElectionResponse =
      ClientRequestError::handle(collector_request)
        .await
        .map_err(|error| ServiceError::RegisterElectionError {
          collector_id: collector.id,
          collector_number: index + 1,
          error,
        })?;

    log::debug!("Got success response from collector {}", index + 1);

    // Update the list of encrypted locations
    create_elections_data.encrypted_locations = collector_response.encryption_result;
  }

  // ==========================================
  //  Database transaction to mark as finished
  // ==========================================
  conn.get().transaction::<_, ServiceError, _>(|| {
    // Create the election itself
    let election = Election::new(
      create_elections_data.id,
      create_elections_data.is_public,
      &create_elections_data.generator,
      &create_elections_data.prime,
    )
    .insert(&conn)?;

    // Add the list of questions
    for question in create_elections_data.questions {
      Question::new(question.id, election.id).insert(&conn)?;
    }

    // Add the list of registered users
    for user_id in create_elections_data.registered_users {
      Registration::new(user_id, election.id).insert(&conn)?;
    }

    // Finally, the list of collectors
    for collector in collectors {
      ElectionCollector::new(election.id, collector.id).insert(&conn)?;
    }

    Ok(())
  })?;

  // Woohoo! Election is now fully initialized!
  Ok(HttpResponse::Ok().finish())
}

///
/// Data sent to each individual collector to initialize election
///
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CollectorCreateElectionData {
  id: Uuid,
  is_public: bool,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  generator: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  prime: BigInt,

  questions: Vec<CreateElectionQuestion>,
  registered_users: Vec<Uuid>,

  num_collectors: usize,
  collector_index: usize,

  /// Use secure two-party multiplication to store location
  ///   If n is provided, then this is on step i, otherwise it is on the last step
  #[serde(with = "kzen_paillier::serialize::vecbigint")]
  encrypted_locations: Vec<BigInt>,
  #[serde(with = "crate::utils::serialize_option_bigint")]
  n: Option<BigInt>,
}

/// Response from each individual collector
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateElectionResponse {
  // Vector might be empty when returning from the second collector
  #[serde(with = "kzen_paillier::serialize::vecbigint")]
  encryption_result: Vec<BigInt>,
}
