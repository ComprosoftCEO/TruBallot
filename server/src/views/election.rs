use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::{Candidate, Election, ElectionStatus, Question, User};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewElectionResult {
  pub id: Uuid,
}

pub type GetElectionByAccessCode = NewElectionResult;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicElectionDetails {
  pub id: Uuid,
  pub name: String,
  pub created_by: CreatedByDetails,
  pub status: ElectionStatus,

  pub is_public: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access_code: Option<String>,

  pub num_registered: i64,
  pub questions: Vec<PublicElectionQuestion>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedByDetails {
  pub id: Uuid,
  pub name: String,
  pub email: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicElectionQuestion {
  pub id: Uuid,
  pub name: String,
  pub num_votes_received: i64,
  pub candidates: Vec<String>,
}

impl PublicElectionDetails {
  pub fn new(
    election: Election,
    created_by: CreatedByDetails,
    num_registered: i64,
    questions: Vec<PublicElectionQuestion>,
  ) -> Self {
    Self {
      id: election.id,
      name: election.name,
      created_by,
      status: election.status,
      is_public: election.is_public,
      access_code: election.access_code,
      num_registered,
      questions,
    }
  }
}

impl CreatedByDetails {
  pub fn new(user: User) -> Self {
    Self {
      id: user.id,
      name: user.name,
      email: user.email,
    }
  }
}

impl PublicElectionQuestion {
  pub fn new(question: Question, num_votes_received: i64, candidates: Vec<Candidate>) -> Self {
    Self {
      id: question.id,
      name: question.question,
      num_votes_received,
      candidates: candidates.into_iter().map(|c| c.candidate).collect(),
    }
  }
}
