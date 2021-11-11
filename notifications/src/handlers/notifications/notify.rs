use actix::Addr;
use actix_web::{web, HttpResponse};

use crate::auth::ServerToken;
use crate::errors::ServiceError;
use crate::notifications::{AllServerMessages, ElectionEvent, GlobalEvent, SubscriptionActor};

pub async fn notify(
  token: ServerToken,
  data: web::Json<AllServerMessages>,
  actor: web::Data<Addr<SubscriptionActor>>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_send_notification()?;

  log::debug!("Broadcast notification: {:#?}", data.0);

  // Handle all of the various notifications
  let addr = actor.as_ref();
  match data.into_inner() {
    AllServerMessages::ElectionCreated(data) => addr.do_send(data.wrap()),
    AllServerMessages::ElectionPublished(data) => addr.do_send(data.wrap()),
    AllServerMessages::NameChanged(data) => addr.do_send(data.wrap()),
    AllServerMessages::ElectionUpdated(data) => addr.do_send(data.wrap()),
    AllServerMessages::ElectionDeleted(data) => addr.do_send(data.wrap()),
    AllServerMessages::RegistrationOpened(data) => addr.do_send(data.wrap()),
    AllServerMessages::UserRegistered(data) => addr.do_send(data.wrap()),
    AllServerMessages::UserUnregistered(data) => addr.do_send(data.wrap()),
    AllServerMessages::RegistrationClosed(data) => addr.do_send(data.wrap()),
    AllServerMessages::VotingOpened(data) => addr.do_send(data.wrap()),
    AllServerMessages::VoteReceived(data) => addr.do_send(data.wrap()),
    AllServerMessages::VotingClosed(data) => addr.do_send(data.wrap()),
    AllServerMessages::ResultsPublished(data) => addr.do_send(data.wrap()),
  }

  Ok(HttpResponse::Ok().finish())
}
