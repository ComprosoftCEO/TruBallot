use actix_web::{web, HttpResponse};
use curv_kzen::{arithmetic::Modulo, BigInt};
use serde::Deserialize;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::auth::ServerToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::Election;
use crate::utils::ConvertBigInt;
use crate::views::election::CancelationShares;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelationSharesData {
  user_id: Uuid,
}

pub async fn get_cancelation_shares(
  token: ServerToken,
  path: web::Path<(Uuid, Uuid)>,
  data: web::Json<CancelationSharesData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  data.validate()?;

  let (election_id, question_id) = path.into_inner();
  let CancelationSharesData { user_id } = data.into_inner();

  // Make sure the election and registration exist
  let election = Election::find_resource(&election_id, &conn)?;
  let registration = election
    .get_registration(&question_id, &user_id, &conn)?
    .ok_or_else(|| ServiceError::UserNotRegistered {
      user_id,
      election_id,
      question_id: Some(question_id),
    })?;

  // Compute the shares
  let modulus = election.prime.to_bigint() - 1;

  let forward_cancelation_shares = BigInt::mod_sub(
    &registration.forward_ballot_shares.to_bigint(),
    &registration.forward_verification_shares.to_bigint(),
    &modulus,
  );

  let reverse_cancelation_shares = BigInt::mod_sub(
    &registration.reverse_ballot_shares.to_bigint(),
    &registration.reverse_verification_shares.to_bigint(),
    &modulus,
  );

  Ok(HttpResponse::Ok().json(CancelationShares {
    forward_cancelation_shares,
    reverse_cancelation_shares,
  }))
}
