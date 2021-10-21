use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus};
use crate::views::election::{PublicElectionDetails, PublicElectionQuestion, UserDetails};

pub async fn get_election(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  token.validate_user_id(&conn)?;

  // Make sure the election exists
  let election = Election::find_resource(&*path, &conn)?;

  // When in the draft state, only the election creator can read the election
  let current_user_id = token.get_user_id();
  if election.status == ElectionStatus::Draft && election.created_by != current_user_id {
    return Err(ServiceError::ElectionNotOwnedByUser {
      current_user_id,
      owner_id: election.created_by,
      action: ResourceAction::ReadPrivate,
    });
  }

  // Otherwise, if the election is private, then can only be read if:
  //   1. Election is in registration phase, or
  //   2. The user is registered for the election
  if !election.is_public {
    if !(election.status == ElectionStatus::Registration || election.is_user_registered(&current_user_id, &conn)?) {
      return Err(NamedResourceType::election(election.id).into_error());
    }
  }

  // Nested details
  let created_by_details = UserDetails::new(election.get_user(&conn)?);
  let registration = election.get_user_registration(&current_user_id, &conn)?;
  let is_registered = registration.is_some();
  let has_voted = registration.map(|r| r.has_voted).unwrap_or(false);

  // Get users registered in the election
  let registrations = election
    .get_registered_users(&conn)?
    .into_iter()
    .map(UserDetails::new)
    .collect();

  // Get all of the questions and candidates
  let mut questions: Vec<PublicElectionQuestion> = Vec::new();
  for (question, candidates) in election.get_questions_candidates_ordered(&conn)? {
    let num_votes_received = question.count_commitments(&conn)?;

    questions.push(PublicElectionQuestion::new(question, num_votes_received, candidates));
  }

  // Build the final result
  let result = PublicElectionDetails::new(
    election,
    created_by_details,
    is_registered,
    has_voted,
    registrations,
    questions,
  );
  Ok(HttpResponse::Ok().json(result))
}
