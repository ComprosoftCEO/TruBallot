use actix::io::{SinkWrite, WriteHandler};
use actix::prelude::*;
use actix_codec::Framed;
use actix_web::web::Bytes;
use awc::{error::WsProtocolError, ws, ws::CloseCode, ws::CloseReason, BoxedSocket};
use curv_kzen::BigInt;
use futures::channel::oneshot;
use futures::stream::{select_all, SplitSink, SplitStream, StreamExt};
use serde::Serialize;
use std::collections::BTreeMap;

use super::types::*;
use super::websocket_messages::*;
use crate::views::verification::VerificationResult;

pub type WsConnection = Framed<BoxedSocket, ws::Codec>;

#[allow(unused)]
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;
type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;

///
/// Mediates communication between all the different websockets
///
pub struct MediatorActor {
  num_collectors: usize,
  websocket_sinks: Vec<SinkWrite<ws::Message, WsFramedSink>>,
  sender: Option<oneshot::Sender<VerificationResult>>,

  // Published ballots
  forward_ballot: BigInt, // Forward Ballot = p_i
  reverse_ballot: BigInt, // Reverse Ballot = p_i'

  // Commitments
  g_s: BigInt,         // g^(s_i)
  g_s_prime: BigInt,   // g^(s_i')
  g_s_s_prime: BigInt, // g^(s_i * s_i')

  // Results
  public_keys: BTreeMap<usize, PublicKey>,
  sp1_result: BTreeMap<usize, bool>,
  sp2_result: BTreeMap<usize, bool>,
}

impl MediatorActor {
  /// Create and start the mediator actor
  ///
  /// This method returns a receiver which indicates the calculation is finished
  pub fn start(
    websocket_connections: Vec<WsConnection>,
    ballot: VerifyBallotData,
  ) -> (Addr<Self>, oneshot::Receiver<VerificationResult>) {
    // Channel to return the result to the API handler when finished
    let (sender, receiver) = oneshot::channel();

    // Build and start the actor
    let actor = Self::create(|ctx| {
      // Split each websocket connection into lists of streams and sinks
      let (streams, sinks): (Vec<_>, Vec<_>) = websocket_connections
        .into_iter()
        .enumerate()
        .map(|(index, connection)| {
          let (sink, stream) = connection.split();
          (stream.map(move |item| (index, item)), SinkWrite::new(sink, ctx))
        })
        .unzip();

      // Add all websockets steams as a single stream to the context
      //
      // Notice that each stream is mapped above to (usize, msg)
      //  This associate each message with the collector index
      ctx.add_stream(select_all(streams));

      // Build the actual actor object
      Self {
        num_collectors: sinks.len(),
        websocket_sinks: sinks,
        sender: Some(sender),

        forward_ballot: ballot.forward_ballot,
        reverse_ballot: ballot.reverse_ballot,
        g_s: ballot.g_s,
        g_s_prime: ballot.g_s_prime,
        g_s_s_prime: ballot.g_s_s_prime,

        public_keys: BTreeMap::new(),
        sp1_result: BTreeMap::new(),
        sp2_result: BTreeMap::new(),
      }
    });

    (actor, receiver)
  }

  /// Verify the signature on a message using the internal public key
  ///
  /// Sends a error close message to the mediator if the signature validation fails
  fn verify_signature<T: SignedMessage>(&self, msg: &T, ctx: &mut <Self as Actor>::Context) -> bool {
    let self_addr = ctx.address();

    // Make sure we actually have the public key for this collector
    //  (This case SHOULD NOT happen in practice)
    let public_key = self.public_keys.get(&msg.get_from());
    if public_key.is_none() {
      self_addr.do_send(ErrorClose::from((
        CloseCode::Abnormal,
        format!(
          "Cannot verify signature, public key for collector {} is not known",
          msg.get_from() + 1,
        ),
      )));

      return false;
    }

    // Test the message signature
    if !msg.verify_signature(public_key.unwrap()) {
      self_addr.do_send(ErrorClose::from((
        CloseCode::Invalid,
        format!("Invalid signature from collector {}", msg.get_from() + 1),
      )));

      return false;
    }

    true
  }

  /// Verifies the "from" field when receiving a message from the collector
  fn verify_origin<T: OriginMessage>(data: &T, collector_index: usize, ctx: &mut <Self as Actor>::Context) -> bool {
    if data.get_from() != collector_index {
      ctx.address().do_send(ErrorClose::from((
        CloseCode::Invalid,
        "Invalid message: \"from\" field does not match the collector",
      )));

      false
    } else {
      true
    }
  }

  /// Send a JSON response back to a given websocket, handling any serialization errors
  fn send_json<T>(&mut self, data: &T, collector_index: usize, ctx: &mut <Self as Actor>::Context)
  where
    T: ?Sized + Serialize,
  {
    match serde_json::to_string(data) {
      Ok(json) => self.text(collector_index, &json),
      Err(e) => ctx
        .address()
        .do_send(ErrorClose::from((CloseCode::Error, format!("{}", e)))),
    }
  }

  /// Close all websockets by sending a close message
  fn close_all_websockets(&mut self, close_reason: &Option<CloseReason>) {
    for collector_index in 0..self.num_collectors {
      self.close(collector_index, close_reason.clone());
    }
  }
}

//
// Various methods to push messages into the websocket stream
//
#[allow(unused)]
impl MediatorActor {
  /// Write a raw frame to the stream
  #[inline]
  fn write_raw(&mut self, collector_index: usize, message: ws::Message) {
    log::debug!("Sending message to collector {}: {:?} ", collector_index + 1, message);

    // Get the sink, trapping any out of bound errors (Should NOT happen)
    let sink = self.websocket_sinks.get_mut(collector_index);
    if sink.is_none() {
      return log::error!(
        "Invalid collector {} (Num Collectors = {})",
        collector_index + 1,
        self.num_collectors
      );
    }

    // Send the message to the websocket, silently failing on an error
    let sink = sink.unwrap();
    if let Some(error) = sink.write(message) {
      return log::error!("Error writing message: {:?}", error);
    }
  }

  /// Send text frame
  #[inline]
  fn text<T: Into<String>>(&mut self, collector_index: usize, text: T) {
    self.write_raw(collector_index, ws::Message::Text(text.into()));
  }

  /// Send binary frame
  #[inline]
  fn binary<B: Into<Bytes>>(&mut self, collector_index: usize, data: B) {
    self.write_raw(collector_index, ws::Message::Binary(data.into()));
  }

  /// Send ping frame
  #[inline]
  fn ping(&mut self, collector_index: usize, message: &[u8]) {
    self.write_raw(collector_index, ws::Message::Ping(Bytes::copy_from_slice(message)));
  }

  /// Send pong frame
  #[inline]
  fn pong(&mut self, collector_index: usize, message: &[u8]) {
    self.write_raw(collector_index, ws::Message::Pong(Bytes::copy_from_slice(message)));
  }

  /// Send close frame
  #[inline]
  fn close(&mut self, collector_index: usize, reason: Option<CloseReason>) {
    self.write_raw(collector_index, ws::Message::Close(reason));
  }
}

///
/// Make this struct into an actor that can run in the background
///
impl Actor for MediatorActor {
  type Context = Context<Self>;

  fn started(&mut self, _ctx: &mut Self::Context) {
    log::info!("Mediator actor started with {} collectors...", self.num_collectors);
  }

  fn stopped(&mut self, _ctx: &mut Self::Context) {
    log::info!("Mediator actor stopped");
  }
}

///
/// Handle any Actix errors when writing to the websocket sink
///
impl WriteHandler<WsProtocolError> for MediatorActor {
  fn error(&mut self, error: WsProtocolError, ctx: &mut Self::Context) -> Running {
    // Send a message to close the actor due to a websocket error
    ctx.address().do_send(ErrorClose::from((
      CloseCode::Error,
      format!("Error writing websocket message: {}", error),
    )));

    Running::Continue
  }
}

///
/// Handler for individual websocket messages received
///
/// Note: This takes (collector_index, msg) to handles messages from multiple websockets
///
impl StreamHandler<(usize, Result<ws::Frame, WsProtocolError>)> for MediatorActor {
  fn handle(&mut self, (collector_index, msg): (usize, Result<ws::Frame, WsProtocolError>), ctx: &mut Self::Context) {
    let self_addr = ctx.address();

    // Handle any errors received from the websocket
    log::debug!("Received message from collector {}: {:#?}", collector_index + 1, msg);
    let msg: ws::Frame = match msg {
      Err(e) => return self_addr.do_send(ErrorClose::from((CloseCode::Error, format!("{}", e)))),
      Ok(msg) => msg,
    };

    // Parse as a JSON string
    let json: WebsocketMessage = match msg {
      // Basic messages
      ws::Frame::Ping(msg) => return self.pong(collector_index, &msg),
      ws::Frame::Pong(_) => return,
      ws::Frame::Close(reason) => {
        log::info!("Received close message, closing all websockets ... ({:#?})", reason);
        (0..self.num_collectors).for_each(|index| self.close(index, reason.clone()));
        return ctx.stop();
      }

      // Parse JSON message
      ws::Frame::Text(text) => match serde_json::from_slice::<WebsocketMessage>(text.as_ref()) {
        Err(e) => return self_addr.do_send(ErrorClose::from((CloseCode::Invalid, format!("Invalid JSON: {}", e)))),
        Ok(json) => json,
      },

      // Unsupported messages
      ws::Frame::Binary(_) => return self_addr.do_send(ErrorClose::from((CloseCode::Unsupported, "Binary Data"))),
      ws::Frame::Continuation(_) => {
        return self_addr.do_send(ErrorClose::from((CloseCode::Unsupported, "Continuation Frame")))
      }
    };

    // Handle the different types of messages that can be received from the collector
    //  (Makes sure to verify the origin for each type of message)
    match json {
      // Special case: must set the from field explicitly
      WebsocketMessage::PublicKey(data) => self_addr.do_send(UnsignedMediatorMessage {
        from: collector_index,
        data,
      }),

      // Verify origin and signature on remaining messages
      WebsocketMessage::SP1_Result_Response(data) => {
        if Self::verify_origin(&data, collector_index, ctx) && self.verify_signature(&data, ctx) {
          self_addr.do_send(data);
        }
      }
      WebsocketMessage::SP2_Result_Response(data) => {
        if Self::verify_origin(&data, collector_index, ctx) && self.verify_signature(&data, ctx) {
          self_addr.do_send(data)
        }
      }
      WebsocketMessage::UnicastMessage(data) => {
        if Self::verify_origin(&data, collector_index, ctx) {
          self_addr.do_send(data)
        }
      }
      WebsocketMessage::BroadcastMessage(data) => {
        if Self::verify_origin(&data, collector_index, ctx) {
          self_addr.do_send(data)
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
/// Close the mediator due to an error
///
impl Handler<ErrorClose> for MediatorActor {
  type Result = ();

  fn handle(&mut self, ErrorClose(code, description): ErrorClose, ctx: &mut Self::Context) -> Self::Result {
    if let Some(ref description) = description {
      log::error!("Closing mediator: {} (Code {:#?})", description, code);
    } else {
      log::error!("Closing mediator: code {:#?}", code);
    }

    // Send the close data to all websockets and stop the actor
    self.close_all_websockets(&Some(CloseReason { code, description }));
    ctx.stop();
  }
}

///
/// Handle the public key being received
///
impl Handler<UnsignedMediatorMessage<PublicKey>> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: UnsignedMediatorMessage<PublicKey>, ctx: &mut Self::Context) -> Self::Result {
    // Save the value into the map
    self.public_keys.insert(msg.from, msg.data);
    self.initialize_if_all_public_keys_received(ctx);
  }
}

impl MediatorActor {
  /// Initialize the websockets if all public keys have been received
  fn initialize_if_all_public_keys_received(&mut self, ctx: &mut <Self as Actor>::Context) {
    if self.public_keys.len() != self.num_collectors {
      return; // Public keys have not all been received
    }

    // Build initialization data to send to all websockets
    let mut data = Initialize {
      collector_index: 0,
      num_collectors: self.num_collectors,

      forward_ballot: self.forward_ballot.clone(),
      reverse_ballot: self.reverse_ballot.clone(),
      g_s: self.g_s.clone(),
      g_s_prime: self.g_s_prime.clone(),
      g_s_s_prime: self.g_s_s_prime.clone(),

      public_keys: self.public_keys.values().cloned().collect(),
    };

    // Initialize all of the websockets
    for collector_index in 0..self.num_collectors {
      data.collector_index = collector_index;
      self.send_json(&data, collector_index, ctx);
    }
  }
}

///
/// Handle the final result for sub-protocol 1
///
impl Handler<SignedMediatorMessage<SP1_Result_Response>> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedMediatorMessage<SP1_Result_Response>, ctx: &mut Self::Context) -> Self::Result {
    // Save the value into the map
    self.sp1_result.insert(msg.from, msg.data.sp1_ballot_valid);
    self.test_if_calculations_finished(ctx);
  }
}

///
/// Handle the final result for sub-protocol 2
///
impl Handler<SignedMediatorMessage<SP2_Result_Response>> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedMediatorMessage<SP2_Result_Response>, ctx: &mut Self::Context) -> Self::Result {
    // Save the value into the map
    self.sp2_result.insert(msg.from, msg.data.sp2_ballot_valid);
    self.test_if_calculations_finished(ctx);
  }
}

impl MediatorActor {
  /// Send the final result if verification has finished
  fn test_if_calculations_finished(&mut self, ctx: &mut <Self as Actor>::Context) {
    if self.sp1_result.len() != self.num_collectors || self.sp2_result.len() != self.num_collectors {
      return;
    }

    log::debug!("Computing final verification result:");
    let sub_protocol_1 = self.sp1_result.values().all(|x| *x);
    let sub_protocol_2 = self.sp2_result.values().all(|x| *x);

    log::debug!(
      "Sub-protocol 1: ballot {}",
      if sub_protocol_1 { "valid" } else { "invalid" }
    );

    log::debug!(
      "Sub-protocol 2: ballot {}",
      if sub_protocol_2 { "valid" } else { "invalid" }
    );

    // Send the verification result back through the channel to the API handler
    let self_addr = ctx.address();
    if let Some(sender) = self.sender.take() {
      if let Err(_) = sender.send(VerificationResult {
        sub_protocol_1,
        sub_protocol_2,
      }) {
        // Handle any errors
        return self_addr.do_send(ErrorClose::from((
          CloseCode::Abnormal,
          "Failed to send verification result, receiver is closed",
        )));
      }
    } else {
      log::error!("Already sent the verification result");
      return self_addr.do_send(ErrorClose::from((
        CloseCode::Abnormal,
        "Failed to send verification result, sender was previously consumed",
      )));
    }

    // Gracefully close all of the websockets, then stop the actor
    self.close_all_websockets(&Some(CloseReason::from(CloseCode::Normal)));
    ctx.stop();
  }
}

///
/// Push a unicast message from one websocket to another
///
impl Handler<SignedUnicastMessage> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedUnicastMessage, ctx: &mut Self::Context) -> Self::Result {
    log::debug!(
      "Forwarding message from collector {} to {}: {:#?}",
      msg.from + 1,
      msg.to + 1,
      msg
    );

    self.send_json(&msg, msg.to, ctx);
  }
}

///
/// Push a broadcast message from one websocket to all websockets (Except itself)
///
impl Handler<SignedBroadcastMessage> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedBroadcastMessage, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Broadcast message from collector {}: {:#?}", msg.from + 1, msg);

    // Broadcast the message to all other websockets (except itself)
    for collector_index in 0..self.num_collectors {
      if collector_index != msg.from {
        self.send_json(&msg, collector_index, ctx);
      }
    }
  }
}
