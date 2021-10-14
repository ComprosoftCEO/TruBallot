use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use curv_kzen::BigInt;
use serde::{Deserialize, Serialize};
use validator::Validate;

use uuid_b64::UuidB64 as Uuid;

use crate::auth::{ClientToken, JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::{ClientRequestError, ServiceError};
use crate::models::{Commitment, Election, ElectionStatus};
use crate::notifications::notify_vote_count_updated;
use crate::utils::ConvertBigInt;
use crate::Collector;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VotingData {
  // Ballots
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_ballot: BigInt, // p_i
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_ballot: BigInt, // p_i'

  // Commitments
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s: BigInt, // g^(s_i)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_prime: BigInt, // g^(s_i')
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_s_prime: BigInt, // g^(s_i * s_i')
}

pub async fn vote(
  token: ClientToken,
  path: web::Path<(Uuid, Uuid)>,
  data: web::Json<VotingData>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_vote()?;
  token.validate_user_id(&conn)?;
  data.validate()?;

  let (election_id, question_id) = path.into_inner();
  let data = data.into_inner();

  // Make sure the election and question exist
  let election = Election::find_resource(&election_id, &conn)?;
  let question = election.find_question(&question_id, &conn)?;

  // Make sure the election is actually open for voting
  if election.status != ElectionStatus::Voting {
    return Err(ServiceError::NotOpenForVoting { election_id });
  }

  // Make sure the user has not already voted
  let user_id = token.get_user_id();
  if let Some(_) = question.find_commitment_optional(&user_id, &conn)? {
    return Err(ServiceError::AlreadyVoted {
      user_id,
      election_id,
      question_id,
    });
  }

  // ================================================
  // Verify the vote with the collectors
  //
  // We can technically call this endpoint on either collector, as both are symmetric.
  // However, for simplicity, always call this on collector 1.
  // Either way, both collectors verify the result, but collector one is the "master".
  // ================================================
  log::debug!("Verifying ballot with the collectors");

  let verify_ballot_data = VerifyBallotData {
    user_id,

    forward_ballot: data.forward_ballot.clone(),
    reverse_ballot: data.reverse_ballot.clone(),

    g_s: data.g_s.clone(),
    g_s_prime: data.g_s_prime.clone(),
    g_s_s_prime: data.g_s_s_prime.clone(),
  };

  let verify_request = Client::builder()
    .disable_timeout()
    .bearer_auth(ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_key.get_encoding_key())?)
    .finish()
    .post(Collector::One.api_url(&format!(
      "/elections/{}/questions/{}/verification",
      election_id, question_id
    ))?)
    .send_json(&verify_ballot_data);

  let VerificationResult {
    sub_protocol_1,
    sub_protocol_2,
  } = ClientRequestError::handle(verify_request)
    .await
    .map_err(|e| ServiceError::RegisterElectionError(Collector::One, e))?;

  // Make sure both sub-protocols are valid
  log::debug!(
    "Sub-protocol 1: ballot {}",
    if sub_protocol_1 { "valid" } else { "invalid" }
  );
  log::debug!(
    "Sub-protocol 2: ballot {}",
    if sub_protocol_2 { "valid" } else { "invalid" }
  );

  if !(sub_protocol_1 && sub_protocol_2) {
    return Err(ServiceError::VoteInvalid {
      sub_protocol_1,
      sub_protocol_2,
    });
  }

  // ================================================
  // Load the data into the database
  // ================================================
  Commitment {
    user_id,
    election_id,
    question_id,

    forward_ballot: data.forward_ballot.to_bigdecimal(),
    reverse_ballot: data.reverse_ballot.to_bigdecimal(),

    g_s: data.g_s.to_bigdecimal(),
    g_s_prime: data.g_s_prime.to_bigdecimal(),
    g_s_s_prime: data.g_s_s_prime.to_bigdecimal(),

    single_vote_verified: sub_protocol_1,
    published_ballots_verified: sub_protocol_2,
  }
  .insert(&conn)?;

  notify_vote_count_updated(&election, &question, &conn, &jwt_key).await;
  log::info!(
    "User {} <{}> cast vote for question {} of \"{}\" <{}>",
    token.get_name(),
    token.get_email(),
    question.question_number + 1,
    election.name,
    election.id
  );

  Ok(HttpResponse::Ok().finish())
}

///
/// JSON structure to send to the collectors to validate the ballot
///
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct VerifyBallotData {
  user_id: Uuid,

  // Ballots
  #[serde(with = "kzen_paillier::serialize::bigint")]
  forward_ballot: BigInt, // p_i
  #[serde(with = "kzen_paillier::serialize::bigint")]
  reverse_ballot: BigInt, // p_i'

  // Commitments
  #[serde(with = "kzen_paillier::serialize::bigint")]
  g_s: BigInt, // g^(s_i)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  g_s_prime: BigInt, // g^(s_i')
  #[serde(with = "kzen_paillier::serialize::bigint")]
  g_s_s_prime: BigInt, // g^(s_i * s_i')
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VerificationResult {
  sub_protocol_1: bool,
  sub_protocol_2: bool,
}
