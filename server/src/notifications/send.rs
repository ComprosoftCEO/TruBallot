use actix_web::client::Client;
use uuid_b64::UuidB64 as Uuid;

use crate::auth::{JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::config;
use crate::db::DbConnection;
use crate::errors::ClientRequestError;
use crate::models::{Commitment, Election, Question, User};
use crate::notifications::{server_types, AllServerMessages};
use crate::utils::ConvertBigInt;

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
pub async fn notify_election_created(election: &Election, creator_id: Uuid, jwt_key: &JWTSecret) {
  send_notification(
    &AllServerMessages::ElectionCreated(server_types::ElectionCreated {
      election_id: election.id,
      creator_id,
    }),
    jwt_key,
  )
  .await
}

pub async fn notify_election_published(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::ElectionPublished(election.id.into()), jwt_key).await
}

pub async fn notify_name_changed(user: &User, jwt_key: &JWTSecret) {
  send_notification(
    &AllServerMessages::NameChanged(server_types::NameChanged {
      user_id: user.id,
      new_name: user.name.clone(),
    }),
    jwt_key,
  )
  .await
}

pub async fn notify_election_updated(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::ElectionUpdated(election.id.into()), jwt_key).await
}

pub async fn notify_election_deleted(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::ElectionDeleted(election.id.into()), jwt_key).await
}

pub async fn notify_registration_opened(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::RegistrationOpened(election.id.into()), jwt_key).await
}

pub async fn notify_user_registered(election: &Election, user_id: &Uuid, conn: &DbConnection, jwt_key: &JWTSecret) {
  let num_registered = match election.count_registrations(conn) {
    Ok(count) => count,
    Err(e) => return log::warn!("Failed to get registration count for notifications: {:#?}", e),
  };

  let user = match User::find(user_id, conn) {
    Ok(user) => user,
    Err(e) => return log::warn!("Failed to get user details for notification: {:#?}", e),
  };

  send_notification(
    &AllServerMessages::UserRegistered(server_types::UserRegistered {
      election_id: election.id,
      user_id: user.id,
      user_name: user.name,
      num_registered,
    }),
    jwt_key,
  )
  .await;
}

pub async fn notify_user_unregistered(election: &Election, user_id: Uuid, conn: &DbConnection, jwt_key: &JWTSecret) {
  let num_registered = match election.count_registrations(conn) {
    Ok(count) => count,
    Err(e) => return log::warn!("Failed to get registration count for notifications: {:#?}", e),
  };

  send_notification(
    &AllServerMessages::UserUnregistered(server_types::UserUnregistered {
      election_id: election.id,
      user_id,
      num_registered,
    }),
    jwt_key,
  )
  .await;
}

pub async fn notify_registration_closed(election: &Election, jwt_key: &JWTSecret) {
  send_notification(
    &AllServerMessages::RegistrationClosed(server_types::RegistrationClosed {
      election_id: election.id,
      is_public: election.is_public,
    }),
    jwt_key,
  )
  .await
}

pub async fn notify_voting_opened(election: &Election, jwt_key: &JWTSecret) {
  send_notification(&AllServerMessages::VotingOpened(election.id.into()), jwt_key).await
}

pub async fn notify_vote_received(
  election: &Election,
  question: &Question,
  commitment: &Commitment,
  conn: &DbConnection,
  jwt_key: &JWTSecret,
) {
  let num_votes = match question.count_commitments(conn) {
    Ok(count) => count,
    Err(e) => return log::warn!("Failed to get vote count for notifications: {:#?}", e),
  };

  let user = match commitment.get_user(conn) {
    Ok(user) => user,
    Err(e) => return log::warn!("Failed to get user details for notification: {:#?}", e),
  };

  let has_voted_status = match election.has_user_voted_status(&user.id, conn) {
    Ok(status) => status,
    Err(e) => return log::warn!("Failed to get has voted status for notification: {:#?}", e),
  };

  send_notification(
    &AllServerMessages::VoteReceived(server_types::VoteReceived {
      election_id: election.id,
      question_id: question.id,

      user_id: user.id,
      user_name: user.name,
      has_voted_status,

      forward_ballot: commitment.forward_ballot.to_bigint(),
      reverse_ballot: commitment.reverse_ballot.to_bigint(),
      g_s: commitment.g_s.to_bigint(),
      g_s_prime: commitment.g_s_prime.to_bigint(),
      g_s_s_prime: commitment.g_s_s_prime.to_bigint(),

      num_votes,
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
