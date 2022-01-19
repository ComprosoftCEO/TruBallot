use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{ClientToken, HasPermission};
use crate::models::Election;
use crate::utils::ConvertBigInt;
use crate::views::election::ElectionParameters;

pub async fn get_election_parameters(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;

  // Make sure the election exists
  let election = Election::find_resource(&*path, &conn)?;
  let encrypted_location = election.get_user_encrypted_location(&token.get_user_id(), &conn)?;

  // Build the final result
  let result = ElectionParameters {
    encrypted_location: encrypted_location.map(|l| l.location.to_bigint()),
  };

  Ok(HttpResponse::Ok().json(result))
}
