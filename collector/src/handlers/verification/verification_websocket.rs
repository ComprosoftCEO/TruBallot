use actix::prelude::*;
use actix_http::ws::{CloseCode, CloseReason};
use actix_web_actors::ws;
use curv_kzen::arithmetic::{Modulo, Samplable};
use curv_kzen::BigInt;
use kzen_paillier::*;
use serde::{Deserialize, Serialize};

use super::websocket_messages::*;
use crate::models::{Election, Question, Registration};
use crate::utils::ConvertBigInt;

/// Actor used for managing the verification protocol
///
/// This protocol verifies both sub-protocol 1 and sub-protocol 2 over websockets.
///
/// The protocol is symmetric as long as we keep track of the "opposite" collector.
/// To make the convention easier, we assume the client is C1 and the server actor is C2
pub struct VerificationWebsocket {
  // Election parameters:
  //   g^x (mod p) is a cyclic group of order p-1
  generator: BigInt,
  prime: BigInt,
  num_registered: i64,
  num_candidates: i64,

  // Published ballots
  p_i: BigInt,       // Forward Ballot = p_i
  p_i_prime: BigInt, // Reverse Ballot = p_i'

  // Commitments
  g_s: BigInt,
  g_s_prime: BigInt,
  g_s_s_prime: BigInt,

  // Shares held by the collector
  s_i_c2: BigInt,           // S_i,C2
  s_i_c2_prime: BigInt,     // S_i,C2'
  stild_i_c2: BigInt,       // S~i,C2
  stild_i_c2_prime: BigInt, // S~i,C2'

  // Secure Two-Party Multiplication (STPM)
  n: BigInt,        // Modulus for Paillier encryption
  r2_prime: BigInt, // STPM: r1 + r2' S_i,C1 * S_i,C2'
  r2: BigInt,       // STPM: r1' + r2 = S_i,C1' * S_i,C2

  // Combined products for Sub-protocol 1
  p1: BigInt,
  p2: BigInt,

  // Sub-protocol 2 values
  g_stild_1: BigInt,       // g^(S~i,C1)
  g_stild_1_prime: BigInt, // g^(S~i,C1')
  g_stild_2: BigInt,       // g^(S~i,C2)
  g_stild_2_prime: BigInt, // g^(S~i,C2')

  g_p_i: BigInt,       // g^(p_i)
  g_p_i_prime: BigInt, // g^(p_i')
}

impl VerificationWebsocket {
  pub fn new(election: Election, question: Question, num_registered: i64, registration: Registration) -> Self {
    let generator = election.generator.to_bigint();
    let prime = election.prime.to_bigint();

    // Extract the shares
    let s_i_c2 = registration.forward_verification_shares.to_bigint(); // S_i,C2
    let s_i_c2_prime = registration.reverse_verification_shares.to_bigint(); // S_i,C2'
    let stild_i_c2 = registration.forward_ballot_shares.to_bigint(); // S~i,C2
    let stild_i_c2_prime = registration.reverse_ballot_shares.to_bigint(); // S~i,C2'

    Self {
      // Election Parameters
      num_registered,
      num_candidates: question.num_candidates,
      generator,
      prime,

      // Ballots  (Don't have this right now)
      p_i: BigInt::from(0),       // Initialized later
      p_i_prime: BigInt::from(0), // Initialized later

      // Commitments (Also don't have this right now)
      g_s: BigInt::from(0),         // Initialized later
      g_s_prime: BigInt::from(0),   // Initialized later
      g_s_s_prime: BigInt::from(0), // Initialized later

      // Shares held by the collector
      s_i_c2,           // S_i,C2
      s_i_c2_prime,     // S_i,C2'
      stild_i_c2,       // S~i,C2
      stild_i_c2_prime, // S~i,C2'

      // Modulus for Paillier encryption (Don't have right now)
      n: BigInt::from(0), // Initialized later

      // r2' and r2 are picked when STPM is actually run
      r2_prime: BigInt::from(0), // Picked later
      r2: BigInt::from(0),       // Picked later

      // Combined products (Can't be computed right now)
      p1: BigInt::from(0), // Computed later
      p2: BigInt::from(0), // Computed later

      // Sub-protocol 2 values
      g_stild_1: BigInt::from(0),       // Computed later
      g_stild_1_prime: BigInt::from(0), // Computed later
      g_stild_2: BigInt::from(0),       // Computed later
      g_stild_2_prime: BigInt::from(0), // Computed later

      g_p_i: BigInt::from(0),       // g^(p_i), Initialized later
      g_p_i_prime: BigInt::from(0), // g^(p_i'), Initialized later
    }
  }
}

/// Close the JSON connection due to an error
#[derive(Message)]
#[rtype(result = "()")]
struct ErrorClose(CloseCode, Option<String>);

impl From<CloseCode> for ErrorClose {
  fn from(code: CloseCode) -> Self {
    Self(code, None)
  }
}

impl<T> From<(CloseCode, T)> for ErrorClose
where
  T: Into<String>,
{
  fn from((code, description): (CloseCode, T)) -> Self {
    Self(code, Some(description.into()))
  }
}

/// Enum of all messages that can be received from the client
#[derive(Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
enum AllClientMessages {
  Initialize(Initialize),
  SP1_STMP1_Request(SP1_STMP1_Request),
  SP1_STMP2_Request(SP1_STMP2_Request),
  SP1_Product1_Request(SP1_Product1_Request),
  SP2_C1_Request(SP2_C1_Request),
}

///
/// VerificationWebsocket Methods
///
impl VerificationWebsocket {
  /// Send a JSON response back to the client, handling any serialization errors
  fn send_json<T>(data: &T, ctx: &mut <Self as Actor>::Context)
  where
    T: ?Sized + Serialize,
  {
    match serde_json::to_string(data) {
      Ok(json) => ctx.text(&json),
      Err(e) => ctx
        .address()
        .do_send(ErrorClose::from((CloseCode::Error, format!("{}", e)))),
    }
  }
}

///
/// Make VerificationWebsocket into an actor that can run in the background
///
impl Actor for VerificationWebsocket {
  type Context = ws::WebsocketContext<Self>;
}

///
/// Handler for individual websocket messages
///
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for VerificationWebsocket {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    let self_addr = ctx.address();

    log::debug!("Received message: {:#?}", msg);
    let msg: ws::Message = match msg {
      Err(e) => return self_addr.do_send(ErrorClose::from((CloseCode::Error, format!("{}", e)))),
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
      ws::Message::Text(text) => match serde_json::from_str::<AllClientMessages>(&text) {
        Err(e) => return self_addr.do_send(ErrorClose::from((CloseCode::Invalid, format!("Invalid JSON: {}", e)))),
        Ok(json) => json,
      },

      // Unsupported messages
      ws::Message::Binary(_) => return self_addr.do_send(ErrorClose::from((CloseCode::Unsupported, "Binary Data"))),
      ws::Message::Continuation(_) => {
        return self_addr.do_send(ErrorClose::from((CloseCode::Unsupported, "Continuation Frame")))
      }
    };

    // Send actor message to handle the data
    match json {
      AllClientMessages::Initialize(data) => self_addr.do_send(data),
      AllClientMessages::SP1_STMP1_Request(data) => self_addr.do_send(data),
      AllClientMessages::SP1_STMP2_Request(data) => self_addr.do_send(data),
      AllClientMessages::SP1_Product1_Request(data) => self_addr.do_send(data),
      AllClientMessages::SP2_C1_Request(data) => self_addr.do_send(data),
    }
  }

  fn finished(&mut self, ctx: &mut Self::Context) {
    log::debug!("Websocket stream closed, stopping actor");
    ctx.stop()
  }
}

///
/// Close the websocket due to an error
///
impl Handler<ErrorClose> for VerificationWebsocket {
  type Result = ();

  fn handle(&mut self, ErrorClose(code, description): ErrorClose, ctx: &mut Self::Context) -> Self::Result {
    if let Some(ref description) = description {
      log::error!("Closing websocket: {} (Code {:#?})", description, code);
    } else {
      log::error!("Closing websocket: code {:#?}", code);
    }

    ctx.close(Some(CloseReason { code, description }));
    ctx.stop();
  }
}

///
/// Initialize the websocket parameters
///
impl Handler<Initialize> for VerificationWebsocket {
  type Result = ();

  fn handle(&mut self, init: Initialize, _ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Initialize parameters");

    // Ballots
    self.p_i = init.forward_ballot;
    self.p_i_prime = init.reverse_ballot;

    // Commitments
    self.g_s = init.g_s;
    self.g_s_prime = init.g_s_prime;
    self.g_s_s_prime = init.g_s_s_prime;

    // STPM Encryption Key
    self.n = init.n;

    // Compute values for sub-protocol 2
    self.g_stild_2 = BigInt::mod_pow(&self.generator, &self.stild_i_c2, &self.prime); // g^(S~i,C2)
    self.g_stild_2_prime = BigInt::mod_pow(&self.generator, &self.stild_i_c2_prime, &self.prime); // g^(S~i,C2')

    self.g_p_i = BigInt::mod_pow(&self.generator, &self.p_i, &self.prime); // g^(p_i)
    self.g_p_i_prime = BigInt::mod_pow(&self.generator, &self.p_i_prime, &self.prime);
  }
}

/// Sub-Protocol 1 - First Secure Two-Party Multiplication Request
///
/// Computes r1 + r2' = S_i,C1 * S_i,C2'
impl Handler<SP1_STMP1_Request> for VerificationWebsocket {
  type Result = ();

  fn handle(&mut self, request: SP1_STMP1_Request, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Sub-protocol 1: First STPM: r1 + r2' = S_i,C1 * S_i,C2'");

    // Pick values for r2'
    self.r2_prime = BigInt::sample_below(&(&self.prime - 1));
    log::debug!("r2' = {}", self.r2_prime);

    // Compute E(r2', e)
    let ek: EncryptionKey = MinimalEncryptionKey { n: self.n.clone() }.into();
    let encrypt_r2_prime: RawCiphertext = Paillier::encrypt(&ek, self.r2_prime.clone().into());

    // Step 2: Compute (E(S_i,C1, e)^(S_i,C2')) * (E(r2', e)^(-1)) (mod n^2)
    let e_s_c1_e_r2_prime = BigInt::mod_mul(
      &BigInt::mod_pow(&request.e_s_c1, &self.s_i_c2_prime, &ek.nn),
      &BigInt::mod_inv(&encrypt_r2_prime.0, &ek.nn).expect("Error: No Inverse"),
      &ek.nn,
    );

    // Send response back to client
    Self::send_json(&SP1_STMP1_Response { e_s_c1_e_r2_prime }, ctx)
  }
}

/// Sub-Protocol 1 - Second Secure Two-Party Multiplication Request
///
/// Computes r1' + r2 = S_i,C1' * S_i,C2
impl Handler<SP1_STMP2_Request> for VerificationWebsocket {
  type Result = ();

  fn handle(&mut self, request: SP1_STMP2_Request, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Sub-protocol 1: Second STPM: r1' + r2 = S_i,C1' * S_i,C2");

    // Pick values for r2
    self.r2 = BigInt::sample_below(&(&self.prime - 1));
    log::debug!("r2 = {}", self.r2);

    // Compute E(r2, e)
    let ek: EncryptionKey = MinimalEncryptionKey { n: self.n.clone() }.into();
    let encrypt_r2: RawCiphertext = Paillier::encrypt(&ek, self.r2.clone().into());

    // Step 2: Compute (E(S_i,C1', e)^(S_i,C2)) * (E(r2, e)^(-1)) (mod n^2)
    let e_s_c1_prime_e_r2 = BigInt::mod_mul(
      &BigInt::mod_pow(&request.e_s_c1_prime, &self.s_i_c2, &ek.nn),
      &BigInt::mod_inv(&encrypt_r2.0, &ek.nn).expect("Error: No Inverse"),
      &ek.nn,
    );

    // Send response back to client
    Self::send_json(&SP1_STMP2_Response { e_s_c1_prime_e_r2 }, ctx)
  }
}

///
/// Sub-Protocol 1 - Computed product P1 request, returns P2
///
impl Handler<SP1_Product1_Request> for VerificationWebsocket {
  type Result = ();

  fn handle(&mut self, request: SP1_Product1_Request, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Sub-protocol 1: Computing combined products P1 and P2");

    // Save the values
    self.p1 = request.p1;
    log::debug!("P1 = {}", self.p1);

    // Compute the product
    let p2 = BigInt::mod_mul(
      &BigInt::mod_mul(
        &BigInt::mod_pow(&self.g_s, &self.s_i_c2_prime, &self.prime),
        &BigInt::mod_pow(&self.g_s_prime, &self.s_i_c2, &self.prime),
        &self.prime,
      ),
      &BigInt::mod_mul(
        &BigInt::mod_pow(&self.generator, &(&self.s_i_c2 * &self.s_i_c2_prime), &self.prime),
        &BigInt::mod_pow(&self.generator, &(&self.r2 + &self.r2_prime), &self.prime),
        &self.prime,
      ),
      &self.prime,
    );
    self.p2 = p2.clone();
    log::debug!("P2 = {}", p2);

    // Send response back
    Self::send_json(&SP1_Product2_Response { p2 }, ctx);

    // Compute the combined product: g^(s_i * s_i') * P1 * P2
    let combined_product = BigInt::mod_mul(
      &self.g_s_s_prime,
      &BigInt::mod_mul(&self.p1, &self.p2, &self.prime),
      &self.prime,
    );
    log::debug!("g^(s_i * s_i') * P1 * P2 = {}", combined_product);

    // Compute the expected product:
    //   g^(2^(L - 1)), where L is the number of bits in the voting vector
    let expected_product = BigInt::mod_pow(
      &self.generator,
      &(BigInt::from(1) << (self.num_registered * self.num_candidates - 1) as usize),
      &self.prime,
    );
    log::debug!("g^(2^(L - 1)) = {}", expected_product);

    // Send the verification result
    let ballot_valid = combined_product == expected_product;
    log::debug!(
      "Sub-protocol 1: ballot {}",
      if ballot_valid { "valid" } else { "invalid" }
    );

    Self::send_json(&SP1_Result_Response { ballot_valid }, ctx);
  }
}

//
/// Sub-Protocol 2 - Computed values g^(S~i,C1) and g^(S~i,C1'), returns g^(S~i,C2) and g^(S~i,C2')
///
impl Handler<SP2_C1_Request> for VerificationWebsocket {
  type Result = ();

  fn handle(&mut self, request: SP2_C1_Request, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Sub-protocol 2 - Compute g values");

    // Save the values
    self.g_stild_1 = request.g_stild_1;
    self.g_stild_1_prime = request.g_stild_1_prime;

    log::debug!("g^(S~i,C1) = {}", self.g_stild_1);
    log::debug!("g^(S~i,C1') = {}", self.g_stild_1_prime);
    log::debug!("g^(S~i,C2) = {}", self.g_stild_2);
    log::debug!("g^(S~i,C2') = {}", self.g_stild_2_prime);

    // Return the collector values
    Self::send_json(
      &SP2_C2_Response {
        g_stild_2: self.g_stild_2.clone(),
        g_stild_2_prime: self.g_stild_2_prime.clone(),
      },
      ctx,
    );

    // Verify the forward ballot
    let g_p_i_combined = BigInt::mod_mul(
      &self.g_s,
      &BigInt::mod_mul(&self.g_stild_1, &self.g_stild_2, &self.prime),
      &self.prime,
    );
    let g_p_i_verified = &g_p_i_combined == &self.g_p_i;

    log::debug!("g^(p_i) = {}", self.g_p_i);
    log::debug!("g^(s_i) * g^(S~i,C1) * g^(S~i,C2) = {}", g_p_i_combined);
    log::debug!("Forward ballot {}", if g_p_i_verified { "valid" } else { "invalid" });

    // Verify the reverse ballot
    let g_p_i_prime_combined = BigInt::mod_mul(
      &self.g_s_prime,
      &BigInt::mod_mul(&self.g_stild_1_prime, &self.g_stild_2_prime, &self.prime),
      &self.prime,
    );
    let g_p_i_prime_verified = &g_p_i_prime_combined == &self.g_p_i_prime;

    log::debug!("g^(p_i') = {}", self.g_p_i_prime);
    log::debug!("g^(s_i') * g^(S~i,C1') * g^(S~i,C2') = {}", g_p_i_prime_combined);
    log::debug!(
      "Reverse ballot {}",
      if g_p_i_prime_verified { "valid" } else { "invalid" }
    );

    // Send the verification result
    let ballot_valid = g_p_i_verified && g_p_i_prime_verified;
    log::debug!(
      "Sub-protocol 2: ballot {}",
      if ballot_valid { "valid" } else { "invalid" }
    );

    Self::send_json(&SP2_Result_Response { ballot_valid }, ctx);

    // Close the connection cleanly
    log::debug!("Both protocols finished, closing websocket cleanly");
    ctx.close(Some(CloseCode::Normal.into()));
    ctx.stop();
  }
}
