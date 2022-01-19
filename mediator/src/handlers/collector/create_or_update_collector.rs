use actix_web::{web, HttpResponse};
use http::Uri;
use serde::Deserialize;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{CollectorToken, HasPermission, JWTSecret};
use crate::models::Collector;
use crate::notifications::notify_collector_created_or_updated;

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
  jwt_key: web::Data<JWTSecret>,
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
  let collector = Collector {
    id,
    name,
    private_base_uri: private_base_uri.to_string(),
    is_secure,
  }
  .insert_or_update(&conn)?;

  log::info!(
    "Collector '{}' (ID: {}) is registered in the database",
    collector.name,
    collector.id
  );
  notify_collector_created_or_updated(&collector, &jwt_key).await;

  Ok(HttpResponse::Ok().finish())
}
