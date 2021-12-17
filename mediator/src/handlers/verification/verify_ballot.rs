use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use futures::future::try_join_all;
use uuid::Uuid as UUID;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use super::mediator_actor::MediatorActor;
use super::types::VerifyBallotData;
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

  // Create all of the collector websocket connections in parallel
  let collectors = election.get_collectors(&conn)?;
  let websocket_connections = try_join_all(collectors.into_iter().map(|collector| {
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

      let connection_stream = WebsocketError::connect(request)
        .await
        .map_err(|e| ServiceError::VerificationError(e))?;

      log::debug!("Success! Websocket open to collector '{}'", collector.name);
      Result::<_, ServiceError>::Ok(connection_stream)
    }
  }))
  .await?;

  // Start actor to handle the websocket communication protocol
  log::info!("Starting mediator actor to handle ballot verification...");
  let (mediator_addr, receiver) = MediatorActor::start(websocket_connections, data);

  // Wait for the calculations to finish
  //   The actor will automatically stop itself
  log::debug!("Beginning protocols and waiting for result...");
  let result: VerificationResult = receiver.await.map_err(|_| ServiceError::VerificationCanceled)?;
  drop(mediator_addr); // Force Rust to not stop the actor prematurely

  // Return final verification results
  log::debug!("Calculations finished, returning final result");
  Ok(HttpResponse::Ok().json(result))
}
