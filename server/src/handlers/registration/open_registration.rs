use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::{ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus};

pub async fn open_registration(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;

  // Find election to make sure it exists in the database
  let mut election = Election::find_resource(&*path, &conn)?;

  // Only the election creator can update the election
  let current_user_id = token.get_user_id();
  if election.created_by != current_user_id {
    return Err(ServiceError::ElectionNotOwnedByUser {
      current_user_id,
      owner_id: election.created_by,
      action: ResourceAction::Update,
    });
  }

  // Make sure the election is still a draft
  if election.status != ElectionStatus::Draft {
    return Err(ServiceError::ElectionNotDraft {
      election_id: election.id,
      action: ResourceAction::Update,
    });
  }

  // Update the status and generate the access code (If applicable)
  election.status = ElectionStatus::Registration;
  if !election.is_public {
    election.generate_unique_access_code(&conn)?;
  }
  election.update(&conn)?;

  Ok(HttpResponse::Ok().finish())
}
