//
// Actor that broadcasts the websocket notifications
//
use actix::prelude::*;
use actix_http::ws::{CloseCode, CloseReason};
use actix_web_actors::ws;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use super::internal_types::{Notify, Replace, Subscribe, Unsubscribe, UnsubscribeAll};
use crate::notifications::{
  client_types::{SubscriptionActions, WebsocketResponse},
  SubscriptionActor,
};

/// Actor used for managing the websocket communication
pub struct WebsocketActor {
  subscription_manager: Addr<SubscriptionActor>,
  user_id: Uuid,
}

impl WebsocketActor {
  pub fn new(subscription_manager: Addr<SubscriptionActor>, user_id: Uuid) -> Self {
    Self {
      subscription_manager,
      user_id,
    }
  }

  /// Send a JSON response back to the client, handling any serialization errors
  fn send_json<T>(data: &T, ctx: &mut <Self as Actor>::Context)
  where
    T: ?Sized + Serialize,
  {
    match serde_json::to_string(data) {
      Ok(json) => ctx.text(&json),
      Err(e) => log::error!("Failed to serialize JSON data: {}", e),
    }
  }
}

/// Close the websocket connection due to an error
#[derive(Message)]
#[rtype(result = "()")]
struct FatalErrorClose(CloseCode, Option<String>);

/// Send back an error response, but keep the websocket open
#[derive(Message)]
#[rtype(result = "()")]
struct NonFatalError {
  message: String,
  developer_notes: Option<String>,
}

impl From<CloseCode> for FatalErrorClose {
  fn from(code: CloseCode) -> Self {
    Self(code, None)
  }
}

impl<T> From<(CloseCode, T)> for FatalErrorClose
where
  T: Into<String>,
{
  fn from((code, description): (CloseCode, T)) -> Self {
    Self(code, Some(description.into()))
  }
}

///
/// Make WebsocketActor into an actor that can run in the background
///
impl Actor for WebsocketActor {
  type Context = ws::WebsocketContext<Self>;

  fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
    // Remove all references to this actor
    self.subscription_manager.do_send(UnsubscribeAll {
      me: ctx.address().recipient(),
    });

    Running::Stop
  }
}

///
/// Handler for individual websocket messages
///
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketActor {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    let self_addr = ctx.address();

    log::debug!("Received message: {:#?}", msg);
    let msg: ws::Message = match msg {
      Err(e) => return self_addr.do_send(FatalErrorClose::from((CloseCode::Error, format!("{}", e)))),
      Ok(msg) => msg,
    };

    // Parse as a JSON string
    let json = match msg {
      // Basic messages
      ws::Message::Nop => return,
      ws::Message::Ping(msg) => return ctx.pong(&msg),
      ws::Message::Pong(_) => return,
      ws::Message::Close(reason) => {
        log::info!("Received close message, closing... ({:#?})", reason);
        ctx.close(reason);
        return ctx.stop();
      }

      // Parse JSON message
      ws::Message::Text(text) => match serde_json::from_str::<SubscriptionActions>(&text) {
        Err(e) => {
          return self_addr.do_send(NonFatalError {
            message: "Invalid JSON data".into(),
            developer_notes: Some(format!("{}", e)),
          })
        }
        Ok(json) => json,
      },

      // Unsupported messages
      ws::Message::Binary(_) => {
        return self_addr.do_send(NonFatalError {
          message: "Unsupported Frame: Binary Data".into(),
          developer_notes: None,
        })
      }
      ws::Message::Continuation(_) => {
        return self_addr.do_send(NonFatalError {
          message: "Unsupported Frame: Continuation".into(),
          developer_notes: None,
        })
      }
    };

    // Send actor message to handle the data
    let me = self_addr.clone().recipient();
    match json {
      SubscriptionActions::Subscribe {
        global_events,
        elections,
        election_events,
      } => self_addr.do_send(Subscribe {
        me,
        global_events,
        elections,
        election_events,
      }),

      SubscriptionActions::Unsubscribe {
        global_events,
        elections,
        election_events,
      } => self_addr.do_send(Unsubscribe {
        me,
        global_events,
        elections,
        election_events,
      }),

      SubscriptionActions::Replace {
        global_events,
        elections,
        election_events,
      } => self_addr.do_send(Replace {
        me,
        global_events,
        elections,
        election_events,
      }),

      SubscriptionActions::UnsubscribeAll => self_addr.do_send(UnsubscribeAll { me }),
    }
  }

  fn finished(&mut self, ctx: &mut Self::Context) {
    log::debug!("Websocket stream closed, stopping actor");
    ctx.stop()
  }
}

//
// Handle websocket errors
//
impl Handler<FatalErrorClose> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, FatalErrorClose(code, description): FatalErrorClose, ctx: &mut Self::Context) -> Self::Result {
    if let Some(ref description) = description {
      log::error!("Closing websocket: {} (Code {:#?})", description, code);
    } else {
      log::error!("Closing websocket: code {:#?}", code);
    }

    ctx.close(Some(CloseReason { code, description }));
    ctx.stop();
  }
}

impl Handler<NonFatalError> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, error: NonFatalError, ctx: &mut Self::Context) -> Self::Result {
    if let Some(ref developer_notes) = error.developer_notes {
      log::error!("{}: {}", error.message, developer_notes);
    } else {
      log::error!("{}", error.message);
    }

    Self::send_json(
      &WebsocketResponse::Error {
        message: error.message,
        developer_notes: error.developer_notes,
      },
      ctx,
    );
  }
}

//
// Handle subscription messages
//
impl Handler<Subscribe> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, subscribe: Subscribe, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Received subscription request: {:#?}", subscribe);

    self.subscription_manager.do_send(subscribe);
    Self::send_json(&WebsocketResponse::Success, ctx);
  }
}

impl Handler<Unsubscribe> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, unsubscribe: Unsubscribe, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Received unsubscribe request: {:#?}", unsubscribe);

    self.subscription_manager.do_send(unsubscribe);
    Self::send_json(&WebsocketResponse::Success, ctx);
  }
}

impl Handler<Replace> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, replace: Replace, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Received replace request: {:#?}", replace);

    self.subscription_manager.do_send(replace);
    Self::send_json(&WebsocketResponse::Success, ctx);
  }
}

impl Handler<UnsubscribeAll> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, unsubscribe_all: UnsubscribeAll, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Received unsubscribe all request");

    self.subscription_manager.do_send(unsubscribe_all);
    Self::send_json(&WebsocketResponse::Success, ctx);
  }
}

//
// Handle notifications
//
impl Handler<Notify> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, notify: Notify, ctx: &mut Self::Context) -> Self::Result {
    match notify.get_protected() {
      None => ctx.text(notify.get_json()),

      Some(user_id) if user_id == self.user_id => ctx.text(notify.get_json()),
      Some(_) => { /* User Id's don't match */ }
    }
  }
}
