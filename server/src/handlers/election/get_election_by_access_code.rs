use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::{Election, ElectionStatus, ACCESS_CODE_LENGTH};
use crate::views::election::GetElectionByAccessCode;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetElectionData {
  pub code: String,
}

pub async fn get_election_by_access_code(
  token: ClientToken,
  query: web::Query<GetElectionData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  token.validate_user_id(&conn)?;

  // Make sure the code length is correct
  let GetElectionData { code } = query.into_inner();
  if code.len() != ACCESS_CODE_LENGTH {
    return Err(ServiceError::AccessCodeNotFound(code));
  }

  // Find the election in the database
  let mut election = match Election::find_access_code(&code, &conn)? {
    None => return Err(ServiceError::AccessCodeNotFound(code)),
    Some(election) => election,
  };

  // Access code is only valid in registration state
  if election.status != ElectionStatus::Registration {
    // Delete the access code, because somehow we have a state invariant in the database
    election.access_code = None;
    election.update(&conn)?;

    return Err(ServiceError::AccessCodeNotFound(code));
  }

  // Code valid! Return the election ID
  Ok(HttpResponse::Ok().json(GetElectionByAccessCode { id: election.id }))
}
