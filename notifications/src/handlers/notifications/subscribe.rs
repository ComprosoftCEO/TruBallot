use actix::Addr;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::auth::ClientToken;
use crate::errors::{ServiceError, WebsocketError};
use crate::notifications::{SubscriptionActor, WebsocketActor};

pub async fn subscribe(
  token: ClientToken,
  actor: web::Data<Addr<SubscriptionActor>>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_subscribe_to_notifications()?;

  // Start the websocket actor to manage notifications
  log::debug!("Starting actor to handle websocket notifications...");
  Ok(
    ws::start(WebsocketActor::new(actor.as_ref().clone()), &req, payload)
      .map_err(|e| ServiceError::NotificationError(WebsocketError::from(e)))?,
  )
}
