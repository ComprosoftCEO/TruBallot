use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::Collector;

/// Special endpoint used by NGINX to proxy a collector path using the UUID.
///
/// This means that the website can use a path like `/api/v1/collector/{ID}/elections`
/// and the path will be proxied to `{COLLECTOR_URL}/api/v1/collector/elections`.
///
/// This works in NGINX by setting the `x-collector-url` header to the base URL path for the collector.
///
/// The NGINX configuration should protect this internal path from the outside world, as it has no
/// authentication to prevent outside calls exposing internal system architecture.
pub async fn proxy_collector(conn: DbConnection, path: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
  let collector = Collector::find_resource(&*path, &conn)?;
  let proxy_url = collector.private_api_url("");

  log::info!("Proxying URL for collector '{}': {}", collector.name, proxy_url);
  Ok(HttpResponse::Ok().header("x-collector-url", proxy_url).finish())
}
