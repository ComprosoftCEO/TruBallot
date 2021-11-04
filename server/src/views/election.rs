use curv_kzen::BigInt;
use serde::Serialize;
use std::collections::HashMap;
use uuid_b64::UuidB64 as Uuid;

use crate::models::{Candidate, Commitment, Election, ElectionStatus, HasVotedStatus, Question, User};
use crate::utils::ConvertBigInt;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewElectionResult {
  pub id: Uuid,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllElectionsResult {
  pub public_elections: Vec<PublicElectionList>,
  pub user_elections: Vec<PublicElectionList>,
  pub registered_elections: Vec<PublicElectionList>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicElectionList {
  pub id: Uuid,
  pub name: String,
  pub status: ElectionStatus,
  pub is_public: bool,
  pub created_by: UserDetails,

  pub is_registered: bool,
  pub has_voted: HasVotedStatus,
  pub num_registered: i64,
  pub num_questions: i64,
}

pub type GetElectionByAccessCode = NewElectionResult;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicElectionDetails {
  pub id: Uuid,
  pub name: String,
  pub created_by: UserDetails,
  pub status: ElectionStatus,

  pub is_public: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access_code: Option<String>,

  pub is_registered: bool,
  pub has_voted: HasVotedStatus,
  pub registered: Vec<RegisteredUserDetails>,
  pub questions: Vec<PublicElectionQuestion>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
  pub id: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredUserDetails {
  pub id: Uuid,
  pub name: String,
  pub has_voted: HasVotedStatus,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicElectionQuestion {
  pub id: Uuid,
  pub name: String,
  pub num_votes_received: i64,
  pub candidates: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishElectionResult {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access_code: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionParameters {
  pub num_registered: i64,
  pub questions: Vec<QuestionParameters>,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub generator: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub prime: BigInt,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionParameters {
  pub num_candidates: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionResult {
  pub question_results: HashMap<Uuid, QuestionResult>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResult {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_ballots: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_ballots: BigInt,
  pub ballot_valid: bool,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_cancelation_shares: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_cancelation_shares: BigInt,

  pub candidate_votes: HashMap<Uuid, CandidateResult>,
  pub user_ballots: Vec<UserBallotResult>,
  pub no_votes: Vec<UserDetails>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBallotResult {
  pub id: Uuid,
  pub name: String,

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
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidateResult {
  // We want to serialize this as NULL if the ballot is invalid
  pub num_votes: Option<i64>,
}

impl PublicElectionList {
  pub fn new(
    election: Election,
    created_by: UserDetails,
    is_registered: bool,
    has_voted: HasVotedStatus,
    num_registered: i64,
    num_questions: i64,
  ) -> Self {
    Self {
      id: election.id,
      name: election.name,
      status: election.status,
      is_public: election.is_public,
      created_by,
      is_registered,
      has_voted,
      num_registered,
      num_questions,
    }
  }
}

impl PublicElectionDetails {
  pub fn new(
    election: Election,
    created_by: UserDetails,
    is_registered: bool,
    has_voted: HasVotedStatus,
    registered: Vec<RegisteredUserDetails>,
    questions: Vec<PublicElectionQuestion>,
  ) -> Self {
    Self {
      id: election.id,
      name: election.name,
      created_by,
      status: election.status,
      is_public: election.is_public,
      access_code: election.access_code,
      is_registered,
      has_voted,
      registered,
      questions,
    }
  }
}

impl UserDetails {
  pub fn new(user: User) -> Self {
    Self {
      id: user.id,
      name: user.name,
    }
  }
}

impl RegisteredUserDetails {
  pub fn new(user: User, has_voted: HasVotedStatus) -> Self {
    Self {
      id: user.id,
      name: user.name,
      has_voted,
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

impl QuestionResult {
  pub fn new(
    question: Question,
    candidate_votes: HashMap<Uuid, CandidateResult>,
    user_ballots: Vec<UserBallotResult>,
    no_votes: Vec<UserDetails>,
  ) -> Self {
    Self {
      forward_ballots: question.final_forward_ballots.to_bigint(),
      reverse_ballots: question.final_reverse_ballots.to_bigint(),
      ballot_valid: question.ballots_valid,

      forward_cancelation_shares: question.forward_cancelation_shares.to_bigint(),
      reverse_cancelation_shares: question.reverse_cancelation_shares.to_bigint(),

      candidate_votes,
      user_ballots,
      no_votes,
    }
  }
}

impl UserBallotResult {
  pub fn new(user: User, commitment: Commitment) -> Self {
    Self {
      id: user.id,
      name: user.name,

      forward_ballot: commitment.forward_ballot.to_bigint(),
      reverse_ballot: commitment.reverse_ballot.to_bigint(),

      g_s: commitment.g_s.to_bigint(),
      g_s_prime: commitment.g_s_prime.to_bigint(),
      g_s_s_prime: commitment.g_s_s_prime.to_bigint(),
    }
  }
}

impl CandidateResult {
  pub fn new(candidate: Candidate) -> Self {
    Self {
      num_votes: candidate.num_votes,
    }
  }
}
