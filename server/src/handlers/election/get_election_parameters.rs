use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus};
use crate::utils::ConvertBigInt;
use crate::views::election::{ElectionParameters, QuestionParameters};

pub async fn get_election_parameters(
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

  // Election must have already closed voting
  if !election.status.is_initialized() {
    return Err(ServiceError::ElectionNotInitialized {
      election_id: election.id,
    });
  }

  // User must be registered for election
  let registration = election.get_user_registration(&token.get_user_id(), &conn)?;

  // If the election is private, then can only be read if user is registered
  if !election.is_public && registration.is_none() {
    return Err(NamedResourceType::election(election.id).into_error());
  }

  // Build the final result
  let questions = election
    .get_questions_candidates(&conn)?
    .into_iter()
    .map(|(_question, candidates)| {
      Ok(QuestionParameters {
        num_candidates: candidates.len() as i64,
      })
    })
    .collect::<Result<Vec<_>, ServiceError>>()?;

  let result = ElectionParameters {
    num_registered: election.count_registrations(&conn)?,
    questions,

    generator: election.generator.to_bigint(),
    prime: election.prime.to_bigint(),

    encryption_key: registration.is_some().then(|| base64::encode(&election.encryption_key)),
    encrypted_location: registration.map(|r| base64::encode(&r.encrypted_location)),
  };

  Ok(HttpResponse::Ok().json(result))
}
