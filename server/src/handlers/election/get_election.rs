use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus};
use crate::views::election::{CreatedByDetails, PublicElectionDetails, PublicElectionQuestion};

pub async fn get_election(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
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
  let created_by_details = CreatedByDetails::new(election.get_user(&conn)?);
  let num_registered = election.count_registrations(&conn)?;

  // Get all of the questions and candidates
  let mut questions: Vec<PublicElectionQuestion> = Vec::new();
  for question in election.get_questions(&conn)? {
    let candidates = question.get_candidates(&conn)?;
    let num_votes_received = question.count_commitments(&conn)?;

    questions.push(PublicElectionQuestion::new(question, num_votes_received, candidates));
  }

  // Build the final result
  let result = PublicElectionDetails::new(election, created_by_details, num_registered, questions);
  Ok(HttpResponse::Ok().json(result))
}
