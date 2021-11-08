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
  pub has_voted_status: HasVotedStatus,
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
  pub has_voted_status: HasVotedStatus,
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
  pub has_voted_status: HasVotedStatus,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicElectionQuestion {
  pub id: Uuid,
  pub name: String,
  pub has_voted: bool,
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
  #[serde(
    with = "crate::utils::serialize_option_bigint",
    skip_serializing_if = "Option::is_none"
  )]
  pub forward_ballots: Option<BigInt>,
  #[serde(
    with = "crate::utils::serialize_option_bigint",
    skip_serializing_if = "Option::is_none"
  )]
  pub reverse_ballots: Option<BigInt>,
  pub ballot_valid: bool,

  #[serde(
    with = "crate::utils::serialize_option_bigint",
    skip_serializing_if = "Option::is_none"
  )]
  pub forward_cancelation_shares: Option<BigInt>,
  #[serde(
    with = "crate::utils::serialize_option_bigint",
    skip_serializing_if = "Option::is_none"
  )]
  pub reverse_cancelation_shares: Option<BigInt>,

  pub user_ballots: Vec<UserBallotResult>,
  pub no_votes: Vec<UserDetails>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub candidate_votes: Option<HashMap<i64, CandidateResult>>,
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
  pub num_votes: i64,
}

impl PublicElectionList {
  pub fn new(
    election: Election,
    created_by: UserDetails,
    is_registered: bool,
    has_voted_status: HasVotedStatus,
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
      has_voted_status,
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
    has_voted_status: HasVotedStatus,
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
      has_voted_status,
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
  pub fn new(user: User, has_voted_status: HasVotedStatus) -> Self {
    Self {
      id: user.id,
      name: user.name,
      has_voted_status,
    }
  }
}

impl PublicElectionQuestion {
  pub fn new(question: Question, has_voted: bool, num_votes_received: i64, candidates: Vec<Candidate>) -> Self {
    Self {
      id: question.id,
      name: question.question,
      has_voted,
      num_votes_received,
      candidates: candidates.into_iter().map(|c| c.candidate).collect(),
    }
  }
}

impl QuestionResult {
  /// Construct a partial election result
  pub fn new_partial(_question: Question, user_ballots: Vec<UserBallotResult>, no_votes: Vec<UserDetails>) -> Self {
    Self {
      forward_ballots: None,
      reverse_ballots: None,
      ballot_valid: false,

      forward_cancelation_shares: None,
      reverse_cancelation_shares: None,

      user_ballots,
      no_votes,
      candidate_votes: None,
    }
  }

  /// Construct a finished election result
  pub fn new(
    question: Question,
    forward_ballots: BigInt,
    reverse_ballots: BigInt,
    candidate_votes: Option<Vec<i64>>,
    user_ballots: Vec<UserBallotResult>,
    no_votes: Vec<UserDetails>,
  ) -> Self {
    let candidate_votes =
      candidate_votes.map(|votes| (0i64..).zip(votes.into_iter().map(CandidateResult::new)).collect());

    Self {
      forward_ballots: Some(forward_ballots),
      reverse_ballots: Some(reverse_ballots),
      ballot_valid: candidate_votes.is_some(),

      forward_cancelation_shares: Some(question.forward_cancelation_shares.to_bigint()),
      reverse_cancelation_shares: Some(question.reverse_cancelation_shares.to_bigint()),

      user_ballots,
      no_votes,
      candidate_votes,
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
  pub fn new(num_votes: i64) -> Self {
    Self { num_votes }
  }
}
