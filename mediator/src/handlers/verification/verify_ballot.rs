use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use futures::future::try_join_all;
use uuid::Uuid as UUID;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use super::mediator_actor::MediatorActor;
use super::types::VerifyBallotData;
use super::websocket_actor::WebsocketActor;
use super::websocket_messages::*;
use crate::auth::{AnyToken, JWTSecret, MediatorToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError, WebsocketError};
use crate::models::{Election, Question};
use crate::views::verification::VerificationResult;

pub async fn verify_ballot(
  token: AnyToken,
  path: web::Path<(Uuid, Uuid)>,
  data: web::Json<VerifyBallotData>,
  conn: DbConnection,
  jwt_secret: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  data.validate()?;

  let (election_id, question_id) = path.into_inner();
  let data = data.into_inner();

  // Make sure the election exist
  let election = Election::find_resource(&election_id, &conn)?;

  // For a private election, make sure the user is registered in the election
  //   Otherwise, they don't have permission to verify the ballot
  let user_id = token.get_user_id();
  if !election.is_public && user_id != UUID::nil().into() {
    let registration = election.get_registration_optional(&user_id, &conn)?;
    if registration.is_none() {
      return Err(NamedResourceType::election(election.id).into_error());
    }
  }

  // Make sure the question and registration exist as well
  let _question = Question::find_resource(&question_id, &election_id, &conn)?;
  let _registration = election
    .get_registration_optional(&data.user_id, &conn)?
    .ok_or_else(|| ServiceError::UserNotRegistered {
      user_id: data.user_id,
      election_id: election.id,
      question_id: Some(question_id),
    })?;

  log::info!("Starting actors to handle ballot verification...");
  let collectors = election.get_collectors(&conn)?;
  let (mediator_addr, receiver) = MediatorActor::start(collectors.len(), &data);

  // Create all of the collector websocket connections in parallel
  let websocket_streams = try_join_all(collectors.into_iter().map(|collector| {
    let jwt_encoding_key = jwt_secret.get_encoding_key();
    let url = collector.private_websocket_url(&format!(
      "/elections/{}/questions/{}/verification/ws/{}",
      election_id, question_id, data.user_id,
    ));

    async move {
      log::debug!("Connecting to collector '{}'...", collector.name);
      let request = Client::builder()
        .disable_timeout()
        .finish()
        .ws(url)
        .bearer_auth(MediatorToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?);

      let stream = WebsocketError::connect(request)
        .await
        .map_err(|e| ServiceError::VerificationError(e))?;

      log::debug!("Success! Websocket open to collector '{}'", collector.name);
      Result::<_, ServiceError>::Ok(stream)
    }
  }))
  .await?;

  // Convert each connection into a running actor
  let websocket_actors: Vec<_> = websocket_streams
    .into_iter()
    .enumerate()
    .map(|(index, stream)| WebsocketActor::start(index, mediator_addr.clone(), stream))
    .collect();

  // Notify the mediator that the websockets are ready
  mediator_addr.do_send(SetWebsocketActors(websocket_actors));

  // Wait for the calculations to finish
  //   The actors will automatically kill themselves
  log::debug!("Beginning protocols and waiting for result...");
  let result: VerificationResult = receiver.await.map_err(|_| ServiceError::VerificationCanceled)?;

  // Return final verification results
  log::debug!("Calculations finished, returning final result");
  Ok(HttpResponse::Ok().json(result))
}
