use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::jwt::{ClientToken, HasPermission};
use crate::models::Election;
use crate::views::collector::PublicCollectorList;

pub async fn get_election_collectors(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_collectors()?;

  let election = Election::find_resource(&*path, &conn)?;

  // For a private election, make sure the user is registered in the election
  //   Otherwise, they don't have permission to view the list of collectors
  //   Exception: The creator is ALWAYS allowed to view the election
  if !election.is_public && election.creator_id != token.get_user_id() {
    let registration = election.get_registration_optional(&token.get_user_id(), &conn)?;
    if registration.is_none() {
      return Err(NamedResourceType::election(election.id).into_error());
    }
  }

  // Okay, get the list!
  let collectors: Vec<_> = election
    .get_collectors_ordered(&conn)?
    .into_iter()
    .map(PublicCollectorList::from_collector)
    .collect();

  Ok(HttpResponse::Ok().json(collectors))
}
