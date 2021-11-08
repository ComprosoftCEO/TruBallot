use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::{Election, ElectionStatus};
use crate::protocol::count_ballot_votes;
use crate::utils::ConvertBigInt;
use crate::views::election::{ElectionResult, QuestionResult, UserBallotResult, UserDetails};

pub async fn get_election_results(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  token.validate_user_id(&conn)?;

  // Make sure the election exists
  let election = Election::find_resource(&*path, &conn)?;

  // If the election is private, then results can only be read:
  //   1. Election is owned by current user, or
  //   2. The user is registered for the election
  if !election.is_public {
    let registration = election.get_user_registration(&token.get_user_id(), &conn)?;
    if !(election.created_by == token.get_user_id() || registration.is_some()) {
      return Err(NamedResourceType::election(election.id).into_error());
    }
  }

  // Election status must allow for viewing voting results
  if !election.status.can_view_results() {
    return Err(ServiceError::ElectionNotStarted {
      election_id: election.id,
    });
  }

  // ===============================================
  //  Gather all details and build the final result
  // ===============================================
  let modulo = election.prime.to_bigint() - 1;
  let num_voters = election.count_registrations(&conn)?;

  let mut question_results: HashMap<Uuid, QuestionResult> = HashMap::new();
  for question in election.get_questions_ordered(&conn)? {
    let question_id = question.id;

    // Get the public commitments for users who DID vote
    let user_ballots: Vec<UserBallotResult> = question
      .get_commitments_users(&conn)?
      .into_iter()
      .map(|(commitment, user)| UserBallotResult::new(user, commitment))
      .collect();

    // Get the list of users who DIDN'T vote
    let no_votes: Vec<UserDetails> = question
      .get_users_without_vote_ordered(&conn)?
      .into_iter()
      .map(UserDetails::new)
      .collect();

    // If the election is finished, then we can safely sum all the ballots to get the final result
    let question_result = if election.status == ElectionStatus::Finished {
      // Compute the sum of all ballots and the cancelation shares
      let (forward_ballots, reverse_ballots) = question.get_ballots_sum(&modulo, &conn)?;

      // Parse the ballots to count the number of votes for each candidates
      //   (This process tests to make sure the voting vector is valid)
      let candidate_votes = count_ballot_votes(
        &forward_ballots,
        &reverse_ballots,
        question.count_candidates(&conn)?,
        num_voters,
        no_votes.len(),
      );

      // Return the full results for the question
      QuestionResult::new(
        question,
        forward_ballots,
        reverse_ballots,
        candidate_votes,
        user_ballots,
        no_votes,
      )
    } else {
      // Otherwise, we don't have all information yet, so return a partial result
      QuestionResult::new_partial(question, user_ballots, no_votes)
    };

    question_results.insert(question_id, question_result);
  }

  Ok(HttpResponse::Ok().json(ElectionResult { question_results }))
}
