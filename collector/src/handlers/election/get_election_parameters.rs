use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::Election;
use crate::views::election::ElectionParameters;

pub async fn get_election_parameters(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;

  // Make sure the election exists
  let election = Election::find_resource(&*path, &conn)?;

  // Build the final result
  let result = ElectionParameters {
    encryption_key: base64::encode(&election.encryption_key),
  };

  Ok(HttpResponse::Ok().json(result))
}
