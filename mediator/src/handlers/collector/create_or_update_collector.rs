use actix_web::{web, HttpResponse};
use http::Uri;
use serde::Deserialize;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::auth::CollectorToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::Collector;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateCollectorData {
  id: Uuid,

  #[validate(length(min = 1))]
  name: String,

  #[serde(with = "http_serde::uri")]
  private_base_uri: Uri,

  is_secure: bool,
}

pub async fn create_or_update_collector(
  token: CollectorToken,
  data: web::Json<CreateUpdateCollectorData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_register_collector()?;
  data.validate()?;

  let CreateUpdateCollectorData {
    id,
    name,
    private_base_uri,
    is_secure,
  } = data.into_inner();

  // Insert into the database, updating values if it already exists
  Collector {
    id,
    name,
    private_base_uri: private_base_uri.to_string(),
    is_secure,
  }
  .insert_or_update(&conn)?;

  Ok(HttpResponse::Ok().finish())
}
