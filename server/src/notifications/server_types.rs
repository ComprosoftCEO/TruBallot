//
// Data structures used to tell the server to broadcast a notification
//
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

/// List of every message that can be deserialized from the server
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AllServerMessages {
  ElectionPublished(ElectionDetails),
  RegistrationOpened(ElectionDetails),
  RegistrationCountUpdated(RegistrationCountUpdated),
  RegistrationClosed(ElectionDetails),
  VotingOpened(ElectionDetails),
  VoteCountUpdated(VoteCountUpdated),
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
pub struct RegistrationCountUpdated {
  pub election_id: Uuid,
  pub num_registered: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteCountUpdated {
  pub election_id: Uuid,
  pub question_id: Uuid,
  pub new_count: i64,
}
