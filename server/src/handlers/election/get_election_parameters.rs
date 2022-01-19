use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::jwt::{ClientToken, HasPermission};
use crate::models::Election;
use crate::utils::ConvertBigInt;
use crate::views::election::{ElectionParameters, QuestionParameters};

pub async fn get_election_parameters(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  token.validate_user_id(&conn)?;

  // Make sure the election exists
  let election = Election::find_resource(&*path, &conn)?;

  // If the election is private, then the parameters can only be read if:
  //   1. Election is owned by current user, or
  //   2. The user is registered for the election
  if !election.is_public {
    let registration = election.get_user_registration(&token.get_user_id(), &conn)?;
    if !(election.created_by == token.get_user_id() || registration.is_some()) {
      return Err(NamedResourceType::election(election.id).into_error());
    }
  }

  // Election parameters must be properly initialized
  if !election.status.is_initialized() {
    return Err(ServiceError::ElectionNotInitialized {
      election_id: election.id,
    });
  }

  // Build the final result
  let questions = election
    .get_questions_candidates_ordered(&conn)?
    .into_iter()
    .map(|(_question, candidates)| {
      Ok(QuestionParameters {
        num_candidates: candidates.len() as i64,
      })
    })
    .collect::<Result<Vec<_>, ServiceError>>()?;

  let result = ElectionParameters {
    num_registered: election.count_registrations(&conn)?,
    questions,

    generator: election.generator.to_bigint(),
    prime: election.prime.to_bigint(),
    location_modulus: election.location_modulus.to_bigint(),
  };

  Ok(HttpResponse::Ok().json(result))
}
