use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::Election;
use crate::utils::ConvertBigInt;
use crate::views::election::QuestionParameters;

pub async fn get_question_parameters(
  token: ClientToken,
  path: web::Path<(Uuid, Uuid)>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;

  let (election_id, question_id) = path.into_inner();

  // Make sure the election exists
  let election = Election::find_resource(&election_id, &conn)?;
  let registration = election
    .get_registration(&question_id, &token.get_user_id(), &conn)?
    .ok_or_else(|| ServiceError::UserNotRegistered {
      user_id: token.get_user_id(),
      election_id,
      question_id: Some(question_id),
    })?;

  // Build the final result
  let result = QuestionParameters {
    forward_verification_shares: registration.forward_verification_shares.to_bigint(),
    reverse_verification_shares: registration.reverse_verification_shares.to_bigint(),
    forward_ballot_shares: registration.forward_ballot_shares.to_bigint(),
    reverse_ballot_shares: registration.reverse_ballot_shares.to_bigint(),
  };

  Ok(HttpResponse::Ok().json(result))
}
