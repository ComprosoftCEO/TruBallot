use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::{Election, ElectionStatus, Registration};

pub async fn register_for_election(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_register_for_election()?;
  token.validate_user_id(&conn)?;

  // Find election to make sure it exists in the database
  let election = Election::find_resource(&*path, &conn)?;

  // Make sure user isn't already registered
  let user_id = token.get_user_id();
  if election.is_user_registered(&user_id, &conn)? {
    return Err(ServiceError::AlreadyRegistered {
      user_id,
      election_id: election.id,
    });
  }

  // Make sure the election is actually open for registration
  if election.status != ElectionStatus::Registration {
    return Err(ServiceError::RegistrationClosed {
      election_id: election.id,
    });
  }

  // Create the new registration in the database
  Registration::new(user_id, election.id).insert(&conn)?;

  Ok(HttpResponse::Ok().finish())
}
