use actix_web::client::Client;
use diesel::prelude::*;

use crate::auth::{JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::config;
use crate::db::DbConnection;
use crate::errors::ClientRequestError;
use crate::models::{Election, Question};
use crate::notifications::{server_types, AllServerMessages};

/// Send a notification, failing silently on an error
async fn send_notification(data: &AllServerMessages, jwt_key: &JWTSecret) {
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
pub async fn notify_election_published(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::ElectionPublished(election.id.into()), jwt_key).await
}

pub async fn notify_registration_opened(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::RegistrationOpened(election.id.into()), jwt_key).await
}

pub async fn notify_registration_count_updated(election: &Election, conn: &DbConnection, jwt_key: &JWTSecret) {
  let num_registered = match election.count_registrations(conn) {
    Ok(count) => count,
    Err(e) => return log::warn!("Failed to get registration count for notifications: {:#?}", e),
  };

  send_notification(
    &AllServerMessages::RegistrationCountUpdated(server_types::RegistrationCountUpdated {
      election_id: election.id,
      num_registered,
    }),
    jwt_key,
  )
  .await;
}

pub async fn notify_registration_closed(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::RegistrationClosed(election.id.into()), jwt_key).await
}

pub async fn notify_voting_opened(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::VotingOpened(election.id.into()), jwt_key).await
}

pub async fn notify_vote_count_updated(
  election: &Election,
  question: &Question,
  conn: &DbConnection,
  jwt_key: &JWTSecret,
) {
  use crate::schema::commitments::dsl::{commitments, election_id, question_id};

  let query = commitments
    .filter(election_id.eq(election.id))
    .filter(question_id.eq(question.id))
    .count();

  let new_count = match query.get_result(conn.get()) {
    Ok(count) => count,
    Err(e) => return log::warn!("Failed to get voting count for notifications: {:#?}", e),
  };

  send_notification(
    &AllServerMessages::VoteCountUpdated(server_types::VoteCountUpdated {
      election_id: election.id,
      question_id: question.id,
      new_count,
    }),
    jwt_key,
  )
  .await;
}

pub async fn notify_voting_closed(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::VotingClosed(election.id.into()), jwt_key).await
}

pub async fn notify_results_published(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::ResultsPublished(election.id.into()), jwt_key).await
}
