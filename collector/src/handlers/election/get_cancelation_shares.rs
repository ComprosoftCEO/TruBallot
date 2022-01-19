use actix_web::{web, HttpResponse};
use curv_kzen::{arithmetic::Modulo, BigInt};
use serde::Deserialize;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{HasPermission, MediatorToken};
use crate::models::Election;
use crate::utils::ConvertBigInt;
use crate::views::election::CancelationShares;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelationSharesData {
  #[validate(length(min = 1))]
  user_ids: Vec<Uuid>,
}

pub async fn get_cancelation_shares(
  token: MediatorToken,
  path: web::Path<(Uuid, Uuid)>,
  data: web::Json<CancelationSharesData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  data.validate()?;

  let (election_id, question_id) = path.into_inner();
  let CancelationSharesData { user_ids } = data.into_inner();

  // Make sure the election and all registrations exist
  let election = Election::find_resource(&election_id, &conn)?;
  let registrations = user_ids
    .into_iter()
    .map(|user_id| {
      Ok(
        election
          .get_registration(&question_id, &user_id, &conn)?
          .ok_or_else(|| ServiceError::UserNotRegistered {
            user_id,
            election_id,
            question_id: Some(question_id),
          })?,
      )
    })
    .collect::<Result<Vec<_>, ServiceError>>()?;

  // Compute the sum of the shares for all users
  let modulus = election.prime.to_bigint() - 1;
  let (forward_cancelation_shares, reverse_cancelation_shares) = registrations.into_iter().fold(
    (BigInt::from(0), BigInt::from(0)),
    |(forward, reverse), registration| {
      (
        BigInt::mod_add(
          &forward,
          &(registration.forward_ballot_shares - registration.forward_verification_shares).to_bigint(),
          &modulus,
        ),
        BigInt::mod_add(
          &reverse,
          &(registration.reverse_ballot_shares - registration.reverse_verification_shares).to_bigint(),
          &modulus,
        ),
      )
    },
  );

  Ok(HttpResponse::Ok().json(CancelationShares {
    forward_cancelation_shares,
    reverse_cancelation_shares,
  }))
}
