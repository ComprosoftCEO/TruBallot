//
// Data structures used to tell the server to broadcast a notification
//
use curv_kzen::BigInt;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::HasVotedStatus;

/// List of every message that can be deserialized from the server
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AllServerMessages {
  ElectionCreated(ElectionCreated),
  ElectionPublished(ElectionDetails),
  NameChanged(NameChanged),
  ElectionUpdated(ElectionDetails),
  ElectionDeleted(ElectionDetails),
  RegistrationOpened(ElectionDetails),
  UserRegistered(UserRegistered),
  UserUnregistered(UserUnregistered),
  RegistrationClosed(RegistrationClosed),
  VotingOpened(VotingOpened),
  VoteReceived(VoteReceived),
  VotingClosed(ElectionDetails),
  ResultsPublished(ElectionDetails),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionDetails {
  pub election_id: Uuid,
}

impl From<Uuid> for ElectionDetails {
  fn from(election_id: Uuid) -> Self {
    Self { election_id }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionCreated {
  pub election_id: Uuid,
  pub creator_id: Uuid,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameChanged {
  pub user_id: Uuid,
  pub new_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRegistered {
  pub election_id: Uuid,
  pub user_id: Uuid,
  pub user_name: String,
  pub num_registered: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUnregistered {
  pub election_id: Uuid,
  pub user_id: Uuid,
  pub num_registered: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationClosed {
  pub election_id: Uuid,
  pub is_public: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingOpened {
  pub election_id: Uuid,
  pub collectors: Vec<Uuid>,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub prime: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub generator: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub location_modulus: BigInt,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteCountUpdated {
  pub election_id: Uuid,
  pub question_id: Uuid,
  pub new_count: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteReceived {
  pub election_id: Uuid,
  pub question_id: Uuid,

  pub user_id: Uuid,
  pub user_name: String,
  pub has_voted_status: HasVotedStatus,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_ballot: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_ballot: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_prime: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_s_prime: BigInt,

  pub num_votes: i64,
}
