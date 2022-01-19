use actix_web::{web, HttpResponse};
use http::Uri;
use serde::Deserialize;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{CollectorToken, HasPermission};
use crate::models::Collector;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectorData {
  #[validate(length(min = 1))]
  name: Option<String>,

  #[serde(with = "crate::utils::serialize_option_uri")]
  private_base_uri: Option<Uri>,

  is_secure: Option<bool>,
}

pub async fn update_collector(
  token: CollectorToken,
  path: web::Path<Uuid>,
  data: web::Json<UpdateCollectorData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_register_collector()?;
  data.validate()?;

  let mut collector = Collector::find_resource(&*path, &conn)?;
  let UpdateCollectorData {
    name,
    private_base_uri,
    is_secure,
  } = data.into_inner();

  // Update the individual details (Only if set in the JSON)
  if let Some(name) = name {
    collector.name = name;
  }
  if let Some(private_base_uri) = private_base_uri {
    collector.private_base_uri = private_base_uri.to_string();
  }
  if let Some(is_secure) = is_secure {
    collector.is_secure = is_secure;
  }

  collector.update(&conn)?;

  Ok(HttpResponse::Ok().finish())
}
