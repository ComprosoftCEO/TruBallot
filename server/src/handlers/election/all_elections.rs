use actix_web::HttpResponse;
use diesel::prelude::*;
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::{Election, ElectionStatus};
use crate::views::election::{AllElectionsResult, PublicElectionList, UserDetails};

pub async fn all_elections(token: ClientToken, conn: DbConnection) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;

  let user = token.validate_user_id(&conn)?;

  let public_elections = get_public_elections(&conn)?;
  let user_elections = user.get_elections_ordered(&conn)?;
  let registered_elections = user.get_registered_elections(&conn)?;

  Ok(HttpResponse::Ok().json(AllElectionsResult {
    public_elections: elections_into_list(public_elections, &user.id, &conn)?,
    user_elections: elections_into_list(user_elections, &user.id, &conn)?,
    registered_elections: elections_into_list(registered_elections, &user.id, &conn)?,
  }))
}

///
/// Public elections are only public once they are out of the draft phase.
/// They continue to stay public, even after registration ends.
///
fn get_public_elections(conn: &DbConnection) -> Result<Vec<Election>, ServiceError> {
  use crate::schema::elections::dsl::{elections, is_public, name, status};

  Ok(
    elections
      .filter(is_public.eq(true))
      .filter(status.ne(ElectionStatus::Draft))
      .order_by(name.asc())
      .get_results::<Election>(conn.get())?,
  )
}

///
/// Convert the elections one-by-one into the list
/// There is probably a more efficient way to do this with SQL queries
///
fn elections_into_list(
  elections: Vec<Election>,
  user_id: &Uuid,
  conn: &DbConnection,
) -> Result<Vec<PublicElectionList>, ServiceError> {
  elections
    .into_iter()
    .map(|election| {
      let num_registered = election.count_registrations(conn)?;
      let num_questions = election.count_questions(conn)?;

      let created_by_details = UserDetails::new(election.get_user(conn)?);
      let registration = election.get_user_registration(user_id, conn)?;
      let is_registered = registration.is_some();
      let has_voted_status = election.has_user_voted_status(user_id, &conn)?;

      Ok(PublicElectionList::new(
        election,
        created_by_details,
        is_registered,
        has_voted_status,
        num_registered,
        num_questions,
      ))
    })
    .collect::<Result<Vec<_>, ServiceError>>()
}
