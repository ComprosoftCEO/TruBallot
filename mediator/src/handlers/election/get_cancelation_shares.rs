use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use curv_kzen::{arithmetic::Modulo, BigInt};
use futures::future::try_join_all;
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use crate::db::DbConnection;
use crate::errors::{ClientRequestError, ServiceError};
use crate::jwt::{HasPermission, JWTSecret, MediatorToken, ServerToken, DEFAULT_PERMISSIONS};
use crate::models::{Collector, Election};
use crate::utils::ConvertBigInt;
use crate::views::election::CancelationShares;

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelationSharesData {
  #[validate(length(min = 1))]
  user_ids: Vec<Uuid>,
}

pub async fn get_cancelation_shares(
  token: ServerToken,
  path: web::Path<(Uuid, Uuid)>,
  data: web::Json<CancelationSharesData>,
  conn: DbConnection,
  jwt_secret: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  data.validate()?;

  let (election_id, question_id) = path.into_inner();
  let data = data.into_inner();

  // Make sure the election, question, and all registrations exist
  let election = Election::find_resource(&election_id, &conn)?;
  election.get_question(&question_id, &conn)?;
  for user_id in data.user_ids.iter() {
    election
      .get_registration_optional(user_id, &conn)?
      .ok_or_else(|| ServiceError::UserNotRegistered {
        user_id: *user_id,
        election_id,
        question_id: Some(question_id),
      })?;
  }

  // Data needed for sending API requests
  let collectors = election.get_collectors(&conn)?;
  let modulus = election.prime.to_bigint() - 1;
  let jwt_encoding_key = jwt_secret.get_encoding_key();

  // Compute the sum of the shares for all users from every collector in the election
  //   Run all requests in parallel to optimize the code
  let (forward_cancelation_shares, reverse_cancelation_shares) =
    try_join_all(collectors.iter().map(|collector| {
      get_cancelation_shares_collector(election_id, question_id, &data, collector, &jwt_encoding_key)
    }))
    .await?
    .into_iter()
    .fold((BigInt::from(0), BigInt::from(0)), |(forward, reverse), result| {
      (
        BigInt::mod_add(&forward, &result.forward_cancelation_shares, &modulus),
        BigInt::mod_add(&reverse, &result.reverse_cancelation_shares, &modulus),
      )
    });

  Ok(HttpResponse::Ok().json(CancelationShares {
    forward_cancelation_shares,
    reverse_cancelation_shares,
  }))
}

///
/// Send request to an individual collector to get the cancelation shares
///
async fn get_cancelation_shares_collector(
  election_id: Uuid,
  question_id: Uuid,
  data: &CancelationSharesData,
  collector: &Collector,
  jwt_encoding_key: &EncodingKey,
) -> Result<CancelationShares, ServiceError> {
  // Build the URL to communicate with the individual collector
  let url = collector.private_api_url(&format!(
    "/elections/{}/questions/{}/cancelation",
    election_id, question_id
  ));

  // Send the request and handle the response
  log::debug!("Request cancelation shares from collector '{}'", collector.name);
  let shares_request = Client::builder()
    .disable_timeout()
    .bearer_auth(MediatorToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
    .finish()
    .get(&url)
    .send_json(data);

  let shares_result: CancelationShares = ClientRequestError::handle(shares_request)
    .await
    .map_err(|e| ServiceError::CancelationSharesError(collector.id, e))?;
  log::debug!("Success! Got cancelation shares from collector '{}'", collector.name);

  // Return the final result
  Ok(shares_result)
}
