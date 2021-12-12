use actix_web::HttpResponse;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::Collector;
use crate::views::collector::PublicCollectorList;

pub async fn all_collectors(token: ClientToken, conn: DbConnection) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_collectors()?;

  let collectors: Vec<_> = Collector::all_ordered(&conn)?
    .into_iter()
    .map(PublicCollectorList::from_collector)
    .collect::<Result<_, ServiceError>>()?;

  Ok(HttpResponse::Ok().json(collectors))
}
