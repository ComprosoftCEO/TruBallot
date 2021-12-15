use actix::io::{SinkWrite, WriteHandler};
use actix::prelude::*;
use actix_codec::Framed;
use actix_web::web::Bytes;
use awc::{error::WsProtocolError, ws, ws::CloseCode, ws::CloseReason, BoxedSocket};
use futures::stream::{SplitSink, SplitStream, StreamExt};
use serde::Serialize;
use std::fmt::Debug;

use super::mediator_actor::MediatorActor;
use super::websocket_messages::*;

pub type WsFramed = Framed<BoxedSocket, ws::Codec>;

#[allow(unused)]
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;
type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;

///
/// Actor that parses and handles all websocket messages from the stream
///
pub struct WebsocketActor {
  collector_index: usize,
  mediator_addr: Addr<MediatorActor>,
  sink: SinkWrite<ws::Message, WsFramedSink>,
}

impl WebsocketActor {
  /// Create and start a new websocket actor given the input stream
  pub fn start(collector_index: usize, mediator_addr: Addr<MediatorActor>, stream: WsFramed) -> Addr<Self> {
    let (sink, stream) = stream.split();
    Self::create(|ctx| {
      ctx.add_stream(stream);
      Self {
        collector_index,
        mediator_addr,
        sink: SinkWrite::new(sink, ctx),
      }
    })
  }

  /// Verifies the "from" field when receiving a packet from the collector
  fn verify_origin<T: OriginMessage>(&self, data: &T, ctx: &mut <Self as Actor>::Context) -> bool {
    if data.get_from() != self.collector_index {
      ctx.address().do_send(WebsocketErrorClose::from((
        CloseCode::Invalid,
        "Invalid message: \"from\" field does not match the collector",
      )));

      false
    } else {
      true
    }
  }

  /// Send a JSON response back to the client, handling any serialization errors
  fn send_json<T>(&mut self, data: &T, ctx: &mut <Self as Actor>::Context)
  where
    T: ?Sized + Serialize,
  {
    match serde_json::to_string(data) {
      Ok(json) => self.text(&json),
      Err(e) => ctx
        .address()
        .do_send(WebsocketErrorClose::from((CloseCode::Error, format!("{}", e)))),
    }
  }
}

//
// Various methods to push messages into the Websocket stream
//
#[allow(unused)]
impl WebsocketActor {
  /// Write a raw frame to the stream
  #[inline]
  fn write_raw(&mut self, message: ws::Message) {
    log::debug!("Sending message {:?}", message);
    if let Some(error) = self.sink.write(message) {
      log::error!("Error writing message: {:?}", error);
    }
  }

  /// Send text frame
  #[inline]
  fn text<T: Into<String>>(&mut self, text: T) {
    self.write_raw(ws::Message::Text(text.into()));
  }

  /// Send binary frame
  #[inline]
  fn binary<B: Into<Bytes>>(&mut self, data: B) {
    self.write_raw(ws::Message::Binary(data.into()));
  }

  /// Send ping frame
  #[inline]
  fn ping(&mut self, message: &[u8]) {
    self.write_raw(ws::Message::Ping(Bytes::copy_from_slice(message)));
  }

  /// Send pong frame
  #[inline]
  fn pong(&mut self, message: &[u8]) {
    self.write_raw(ws::Message::Pong(Bytes::copy_from_slice(message)));
  }

  /// Send close frame
  #[inline]
  fn close(&mut self, reason: Option<CloseReason>) {
    self.write_raw(ws::Message::Close(reason));
  }
}

///
/// Make this struct into an actor that can run in the background
///
impl Actor for WebsocketActor {
  type Context = Context<Self>;

  fn started(&mut self, _ctx: &mut Self::Context) {
    log::info!("Websocket actor started for collector {}...", self.collector_index + 1);
  }

  fn stopped(&mut self, _ctx: &mut Self::Context) {
    log::info!("Websocket actor stopped for collector {}", self.collector_index + 1);
  }
}

/// Required for Actix to work with the stream
impl WriteHandler<WsProtocolError> for WebsocketActor {}

///
/// Handler for individual websocket messages
///
impl StreamHandler<Result<ws::Frame, WsProtocolError>> for WebsocketActor {
  fn handle(&mut self, msg: Result<ws::Frame, WsProtocolError>, ctx: &mut Self::Context) {
    let self_addr = ctx.address();

    log::debug!("Received message: {:#?}", msg);
    let msg: ws::Frame = match msg {
      Err(e) => return self_addr.do_send(WebsocketErrorClose::from((CloseCode::Error, format!("{}", e)))),
      Ok(msg) => msg,
    };

    // Parse as a JSON string
    let json: WebsocketMessage = match msg {
      // Basic messages
      ws::Frame::Ping(msg) => return self.pong(&msg),
      ws::Frame::Pong(_) => return,
      ws::Frame::Close(reason) => {
        log::info!("Received close message, closing... ({:#?})", reason);
        self.close(reason);
        return ctx.stop();
      }

      // Parse JSON message
      ws::Frame::Text(text) => match serde_json::from_slice::<WebsocketMessage>(text.as_ref()) {
        Err(e) => {
          return self_addr.do_send(WebsocketErrorClose::from((
            CloseCode::Invalid,
            format!("Invalid JSON: {}", e),
          )))
        }
        Ok(json) => json,
      },

      // Unsupported messages
      ws::Frame::Binary(_) => {
        return self_addr.do_send(WebsocketErrorClose::from((CloseCode::Unsupported, "Binary Data")))
      }
      ws::Frame::Continuation(_) => {
        return self_addr.do_send(WebsocketErrorClose::from((
          CloseCode::Unsupported,
          "Continuation Frame",
        )))
      }
    };

    // Handle the different types of messages
    match json {
      // Special case for the public key, must set the "from" field when forwarding the message
      WebsocketMessage::PublicKey(public_key) => self.mediator_addr.do_send(UnsignedMediatorMessage {
        from: self.collector_index,
        data: public_key.data,
      }),

      // Remaining messages need to verify the origin
      WebsocketMessage::SP1_Result_Response(data) => {
        if self.verify_origin(&data, ctx) {
          self.mediator_addr.do_send(data);
        }
      }
      WebsocketMessage::SP2_Result_Response(data) => {
        if self.verify_origin(&data, ctx) {
          self.mediator_addr.do_send(data)
        }
      }
      WebsocketMessage::UnicastMessage(data) => {
        if self.verify_origin(&data, ctx) {
          self.mediator_addr.do_send(data);
        }
      }
      WebsocketMessage::BroadcastMessage(data) => {
        if self.verify_origin(&data, ctx) {
          self.mediator_addr.do_send(data);
        }
      }
    }
  }

  fn finished(&mut self, ctx: &mut Self::Context) {
    log::debug!("Websocket stream closed, stopping actor");
    ctx.stop()
  }
}

///
/// Close the websocket gracefully when all connections are donw
///
impl Handler<CloseWebsocketGracefully> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, _: CloseWebsocketGracefully, ctx: &mut Self::Context) -> Self::Result {
    log::info!(
      "Received close message from mediator, closing websocket gracefully for collector {}",
      self.collector_index + 1
    );

    // Send the websocket close frame and stop the actor
    //  No need to close the mediator, as this message comes when the mediator is shutting down
    self.close(Some(CloseReason {
      code: CloseCode::Normal,
      description: None,
    }));
    ctx.stop();
  }
}

///
/// Close the websocket due to an error
///
impl Handler<WebsocketErrorClose> for WebsocketActor {
  type Result = ();

  fn handle(
    &mut self,
    WebsocketErrorClose(code, description): WebsocketErrorClose,
    ctx: &mut Self::Context,
  ) -> Self::Result {
    if let Some(ref description) = description {
      log::error!("Closing websocket: {} (Code {:#?})", description, code);
    } else {
      log::error!("Closing websocket: code {:#?}", code);
    }

    // Close the mediator
    self
      .mediator_addr
      .do_send(MediatorErrorClose::from("Websocket closed unexpectedly"));

    // Send the websocket close frame and stop the actor
    self.close(Some(CloseReason { code, description }));
    ctx.stop();
  }
}

///
/// Push a message from the mediator
///
impl<T: Debug + Serialize> Handler<MediatorMessage<T>> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: MediatorMessage<T>, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Forwarding message: {:#?}", msg);
    self.send_json(&msg, ctx);
  }
}

///
/// Push a unicast message from another websocket
///
impl Handler<SignedUnicastMessage> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: SignedUnicastMessage, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Forwarding message: {:#?}", msg);
    self.send_json(&msg, ctx);
  }
}

///
/// Push a broadcast message from another websocket
///
impl Handler<SignedBroadcastMessage> for WebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: SignedBroadcastMessage, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Forwarding message: {:#?}", msg);
    self.send_json(&msg, ctx);
  }
}
