use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::Collector;
use crate::views::collector::PublicCollector;

pub async fn get_collector(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_collectors()?;

  let collector = Collector::find_resource(&*path, &conn)?;
  let result = PublicCollector::from_collector(collector)?;

  Ok(HttpResponse::Ok().json(result))
}
