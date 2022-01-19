use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{ClientToken, HasPermission, JWTSecret};
use crate::models::{Election, ElectionStatus};
use crate::notifications::notify_user_unregistered;

pub async fn unregister_from_election(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_register_for_election()?;
  token.validate_user_id(&conn)?;

  // Find election to make sure it exists in the database
  let election = Election::find_resource(&*path, &conn)?;

  // Make sure the election is actually open for registration
  if election.status != ElectionStatus::Registration {
    return Err(ServiceError::RegistrationClosed {
      election_id: election.id,
    });
  }

  // Make sure user is already registered
  let user_id = token.get_user_id();
  let registration = match election.get_user_registration(&user_id, &conn)? {
    Some(registration) => registration,
    None => {
      return Err(ServiceError::NotRegistered {
        user_id,
        election_id: election.id,
      });
    },
  };

  // Delete the registration from the database
  registration.delete(&conn)?;
  notify_user_unregistered(&election, user_id, &conn, &jwt_key).await;

  Ok(HttpResponse::Ok().finish())
}
