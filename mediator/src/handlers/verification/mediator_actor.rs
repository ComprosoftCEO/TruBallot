use actix::prelude::*;
use awc::ws::CloseCode;
use curv_kzen::BigInt;
use futures::channel::oneshot;
use std::collections::BTreeMap;

use super::types::*;
use super::websocket_actor::WebsocketActor;
use super::websocket_messages::*;
use crate::views::verification::VerificationResult;

///
/// Mediates communication between all the different websocket actors
///
pub struct MediatorActor {
  num_collectors: usize,
  websocket_connections: Vec<Addr<WebsocketActor>>,
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
    num_collectors: usize,
    ballot: &VerifyBallotData,
  ) -> (Addr<Self>, oneshot::Receiver<VerificationResult>) {
    let (sender, receiver) = oneshot::channel();
    let actor = Self {
      num_collectors,
      websocket_connections: Vec::new(),
      sender: Some(sender),

      forward_ballot: ballot.forward_ballot.clone(),
      reverse_ballot: ballot.reverse_ballot.clone(),
      g_s: ballot.g_s.clone(),
      g_s_prime: ballot.g_s_prime.clone(),
      g_s_s_prime: ballot.g_s_s_prime.clone(),

      public_keys: BTreeMap::new(),
      sp1_result: BTreeMap::new(),
      sp2_result: BTreeMap::new(),
    };

    (actor.start(), receiver)
  }

  /// Verify the signature on a message using the internal public key
  ///
  /// Sends a error close message to the mediator if the signature validation fails
  fn verify_signature<T: SignedMessage>(&self, msg: &T, ctx: &mut <Self as Actor>::Context) -> bool {
    let self_addr = ctx.address();
    match self.public_keys.get(&msg.get_from()) {
      Some(public_key) if msg.verify_signature(public_key) => true,
      Some(_) => {
        self_addr.do_send(MediatorErrorClose::from(format!(
          "Invalid signature from collector {}",
          msg.get_from() + 1
        )));
        false
      }
      None => {
        self_addr.do_send(MediatorErrorClose::from(format!(
          "Cannot verify signature, public key for collector {} is not known",
          msg.get_from() + 1,
        )));
        false
      }
    }
  }
}

///
/// Make this struct into an actor that can run in the background
///
impl Actor for MediatorActor {
  type Context = Context<Self>;

  fn started(&mut self, _ctx: &mut Self::Context) {
    log::info!("Mediator actor started...");
  }

  fn stopped(&mut self, _ctx: &mut Self::Context) {
    log::info!("Mediator actor stopped");
  }
}

///
/// Close the mediator actor due to an error
///
impl Handler<MediatorErrorClose> for MediatorActor {
  type Result = ();

  fn handle(&mut self, MediatorErrorClose(description): MediatorErrorClose, ctx: &mut Self::Context) -> Self::Result {
    if let Some(ref description) = description {
      log::error!("Closing mediator: {}", description);
    } else {
      log::error!("Closing mediator due to an unexpected error");
    }

    // Send close messages to all websockets
    for websocket in self.websocket_connections.iter() {
      websocket.do_send(WebsocketErrorClose::from(CloseCode::Error));
    }

    // Stop the actor!
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

///
/// Handle all other related actors being initialized
///
impl Handler<SetWebsocketActors> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SetWebsocketActors, ctx: &mut Self::Context) -> Self::Result {
    self.websocket_connections = msg.0;
    self.initialize_if_all_public_keys_received(ctx);
  }
}

impl MediatorActor {
  /// Initialize the websockets if all public keys have been received
  ///
  /// This requires the following 2 conditions to be met:
  ///   1. All websockets are established to the collectors
  ///   2. All public keys have been received from the collectors
  fn initialize_if_all_public_keys_received(&mut self, _ctx: &mut <Self as Actor>::Context) {
    if self.websocket_connections.len() != self.num_collectors {
      return; // Websockets are not all initialized
    }

    if self.public_keys.len() != self.num_collectors {
      return; // Public keys have not all been received
    }

    // Initialize all of the websockets
    let public_keys: Vec<_> = self.public_keys.values().cloned().collect();
    for (index, websocket) in self.websocket_connections.iter().enumerate() {
      let data = Initialize {
        collector_index: index,
        num_collectors: self.num_collectors,

        forward_ballot: self.forward_ballot.clone(),
        reverse_ballot: self.reverse_ballot.clone(),
        g_s: self.g_s.clone(),
        g_s_prime: self.g_s_prime.clone(),
        g_s_s_prime: self.g_s_s_prime.clone(),

        public_keys: public_keys.clone(),
      };

      websocket.do_send(MediatorMessage { data });
    }
  }
}

///
/// Handle the final result for sub-protocol 1
///
impl Handler<SignedMediatorMessage<SP1_Result_Response>> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedMediatorMessage<SP1_Result_Response>, ctx: &mut Self::Context) -> Self::Result {
    if !self.verify_signature(&msg, ctx) {
      return;
    }

    // Save the value into the map
    self.sp1_result.insert(msg.from, msg.data.sp1_ballot_valid);
    self.initialize_if_all_public_keys_received(ctx);
  }
}

///
/// Handle the final result for sub-protocol 2
///
impl Handler<SignedMediatorMessage<SP2_Result_Response>> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedMediatorMessage<SP2_Result_Response>, ctx: &mut Self::Context) -> Self::Result {
    if !self.verify_signature(&msg, ctx) {
      return;
    }

    // Save the value into the map
    self.sp2_result.insert(msg.from, msg.data.sp2_ballot_valid);
    self.test_if_calculations_finished(ctx);
  }
}

impl MediatorActor {
  /// Send the final result if verification has finished
  fn test_if_calculations_finished(&mut self, ctx: &mut <Self as Actor>::Context) {
    if self.sp1_result.len() != self.num_collectors && self.sp2_result.len() != self.num_collectors {
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

    let self_addr = ctx.address();
    if let Some(sender) = self.sender.take() {
      if let Err(_) = sender.send(VerificationResult {
        sub_protocol_1,
        sub_protocol_2,
      }) {
        // Handle any errors
        return self_addr.do_send(MediatorErrorClose::from(
          "Failed to send verification result, receiver is closed",
        ));
      }
    } else {
      log::error!("Already sent the verification result");
      return self_addr.do_send(MediatorErrorClose::from(
        "Failed to send verification result, sender was previously consumed",
      ));
    }

    // Gracefully close all of the websockets
    for websocket in self.websocket_connections.iter() {
      websocket.do_send(CloseWebsocketGracefully);
    }

    // Then stop the actor gracefully
    ctx.stop();
  }
}

///
/// Push a unicast message from another websocket
///
impl Handler<SignedUnicastMessage> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedUnicastMessage, ctx: &mut Self::Context) -> Self::Result {
    match self.websocket_connections.get(msg.to) {
      Some(addr) => addr.do_send(msg),
      None => {
        return ctx.address().do_send(MediatorErrorClose::from(format!(
          "Cannot forward unicast message to collector {}, not registered in system",
          msg.to + 1
        )));
      }
    }
  }
}

///
/// Push a broadcast message from another websocket
///
impl Handler<SignedBroadcastMessage> for MediatorActor {
  type Result = ();

  fn handle(&mut self, msg: SignedBroadcastMessage, _ctx: &mut Self::Context) -> Self::Result {
    // Broadcast the message to all other websockets (except itself)
    for (index, addr) in self.websocket_connections.iter().enumerate() {
      if index != msg.from {
        addr.do_send(msg.clone());
      }
    }
  }
}
