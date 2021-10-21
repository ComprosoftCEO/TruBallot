use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::{Election, ElectionStatus};
use crate::views::election::{CandidateResult, ElectionResult, QuestionResult, UserBallotResult, UserDetails};

pub async fn get_election_results(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  token.validate_user_id(&conn)?;

  // Make sure the election exists
  let election = Election::find_resource(&*path, &conn)?;

  // If the election is private, then results can only be read if user is registered
  if !election.is_public && election.get_user_registration(&token.get_user_id(), &conn)?.is_none() {
    return Err(NamedResourceType::election(election.id).into_error());
  }

  // Election must have already closed voting
  if election.status != ElectionStatus::Finished {
    return Err(ServiceError::ElectionNotFinished {
      election_id: election.id,
    });
  }

  // Build the final result
  let mut question_results: HashMap<Uuid, QuestionResult> = HashMap::new();
  for (question, candidates) in election.get_questions_candidates_ordered(&conn)? {
    let candidate_votes: HashMap<Uuid, CandidateResult> = candidates
      .into_iter()
      .map(|candidate| (candidate.id, CandidateResult::new(candidate)))
      .collect();

    let user_ballots: Vec<UserBallotResult> = question
      .get_commitments_users(&conn)?
      .into_iter()
      .map(|(commitment, user)| UserBallotResult::new(user, commitment))
      .collect();

    let no_votes: Vec<UserDetails> = question
      .get_users_without_vote_ordered(&conn)?
      .into_iter()
      .map(UserDetails::new)
      .collect();

    question_results.insert(
      question.id,
      QuestionResult::new(question, candidate_votes, user_ballots, no_votes),
    );
  }

  Ok(HttpResponse::Ok().json(ElectionResult { question_results }))
}
