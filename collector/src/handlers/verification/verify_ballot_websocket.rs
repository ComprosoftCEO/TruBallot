use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid_b64::UuidB64 as Uuid;

use super::verification_websocket_actor::VerificationWebsocketActor;
use crate::db::DbConnection;
use crate::errors::{ServiceError, WebsocketError};
use crate::jwt::{HasPermission, MediatorToken};
use crate::models::{Election, Question};

pub async fn verify_ballot_websocket(
  token: MediatorToken,
  path: web::Path<(Uuid, Uuid, Uuid)>,
  conn: DbConnection,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;

  let (election_id, question_id, user_id) = path.into_inner();

  // Make sure the election, question, and user registration exist
  let election = Election::find_resource(&election_id, &conn)?;
  let question = Question::find_resource(&question_id, &election_id, &conn)?;
  let num_registered = question.count_registrations(&conn)?;
  let registration = election
    .get_registration(&question_id, &user_id, &conn)?
    .ok_or_else(|| ServiceError::UserNotRegistered {
      user_id,
      election_id: election.id,
      question_id: Some(question_id),
    })?;

  // Start the websocket server
  log::debug!("Starting actor to serve verification websocket...");
  Ok(
    ws::start(
      VerificationWebsocketActor::new(election, question, num_registered, registration),
      &req,
      payload,
    )
    .map_err(|e| ServiceError::VerificationError(WebsocketError::from(e)))?,
  )
}
