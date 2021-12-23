use actix_web::client::Client;

use crate::auth::{JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::config;
use crate::errors::ClientRequestError;
use crate::models::Collector;
use crate::notifications::{mediator_types, AllMediatorMessages};

/// Send a notification, failing silently on an error
async fn send_notification(data: &AllMediatorMessages, jwt_key: &JWTSecret) {
  // Try to get the URL
  let url = match config::get_notifications_url() {
    Some(url) => url,
    None => return log::warn!("Notifications URL is not set, unable to send notification"),
  };

  let jwt = match ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_key.get_encoding_key()) {
    Ok(token) => token,
    Err(e) => return log::warn!("Failed to encode JWT for sending notifications: {:#?}", e),
  };

  let request = Client::builder()
    .disable_timeout()
    .bearer_auth(jwt)
    .finish()
    .post(&format!("{}/api/v1/notifications", url))
    .send_json(data);

  match ClientRequestError::handle_empty(request).await {
    Ok(()) => log::debug!("Sent notification: {:#?}", data),
    Err(e) => log::warn!("Failed to send notification: {:#?}", e),
  }
}

//
// Methods to send the notifications to the server
//
pub async fn notify_collector_created_or_updated(collector: &Collector, jwt_key: &JWTSecret) {
  send_notification(
    &AllMediatorMessages::CollectorPublishedOrUpdated(mediator_types::CollectorPublishedOrUpdated {
      id: collector.id,
      name: collector.name.clone(),
    }),
    jwt_key,
  )
  .await
}
