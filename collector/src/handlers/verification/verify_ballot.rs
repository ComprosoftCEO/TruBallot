use actix_web::{web, HttpResponse};
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use super::VerifyBallotData;
use crate::auth::AnyToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::{Election, Question, Registration};
use crate::protocol::SharesMatrix;
use crate::utils::ConvertBigInt;
use crate::views::election::CreateElectionResponse;
use crate::Collector;

pub async fn verify_ballot(
  token: AnyToken,
  path: web::Path<(Uuid, Uuid)>,
  data: web::Json<VerifyBallotData>,
  collector: web::Data<Collector>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  data.validate()?;

  let (election_id, question_id) = path.into_inner();

  // Make sure the election and user registration exist
  let election = Election::find_resource(&election_id, &conn)?;
  let registration = election
    .get_registration(&question_id, &data.user_id, &conn)?
    .ok_or_else(|| ServiceError::UserNotRegistered {
      user_id: data.user_id,
      election_id: election.id,
    })?;

  Ok(HttpResponse::Ok().finish())
}
