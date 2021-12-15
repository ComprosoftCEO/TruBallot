use actix::prelude::*;
use actix_http::ws::{CloseCode, CloseReason};
use actix_web_actors::ws;
use curv_kzen::arithmetic::{Modulo, Samplable};
use curv_kzen::BigInt;
use serde::Serialize;
use std::collections::BTreeMap;
use std::hash::Hash;

use super::websocket_messages::*;
use crate::models::{Election, Question, Registration};
use crate::protocol::stpm;
use crate::utils::ConvertBigInt;

/// Actor used for managing the verification protocol
///
/// This protocol verifies both sub-protocol 1 and sub-protocol 2 over websockets.
pub struct VerificationWebsocketActor {
  // Collector details
  collector_index: usize,
  num_collectors: usize,
  public_keys: Vec<PublicKey>,

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

  // Private key (Paillier cryptosystem and RSA)
  n: BigInt, // p * q
  paillier_p: BigInt,
  paillier_q: BigInt,
  rsa_a: BigInt,
  rsa_b: BigInt,

  // Shares held by the collector
  s_i_cj: BigInt,           // S_i,Cj
  s_i_cj_prime: BigInt,     // S_i,Cj'
  stild_i_cj: BigInt,       // S~i,Cj
  stild_i_cj_prime: BigInt, // S~i,Cj'

  // STPM values for Sub-protocol 1
  sp1_values: BTreeMap<usize, SP1_STPM_Values>,

  // Combined products for Sub-protocol 1
  products: BTreeMap<usize, BigInt>,

  // Sub-protocol 2 values
  sp2_values: BTreeMap<usize, SP2_Shares_Response>,
}

/// Values needed to store sub-protocol 1 STPM values for collector j
#[allow(non_camel_case_types)]
struct SP1_STPM_Values {
  r: BigInt,
  r_prime: BigInt,
}

///
/// VerificationWebsocket Methods
///
impl VerificationWebsocketActor {
  /// Create a new actor to handle websocket verification
  pub fn new(election: Election, question: Question, num_registered: i64, registration: Registration) -> Self {
    let generator = election.generator.to_bigint();
    let prime = election.prime.to_bigint();

    // Private key for paillier cryptosystem
    let paillier_p = election.paillier_p.to_bigint();
    let paillier_q = election.paillier_q.to_bigint();

    // Generate public-private key pair for RSA:
    //  Let n = p*q, where p and q are safe primes
    //  Also Φ(n) = (p-1)(q-1) is the Euler totient function
    //
    //  Pick values a and b such that a*b ≡ 1 (mod Φ(n))
    //
    //  Public Key: (b, n)     Private key: (a)
    //  E(x) = x^b (mod n)     D(x) = E(x)^a (mod n)
    let totient = (&paillier_p - 1) * (&paillier_q - 1);
    let (rsa_a, rsa_b) = loop {
      let a = BigInt::sample_below(&totient);
      if let Some(b) = BigInt::mod_inv(&a, &totient) {
        break (a, b);
      }
    };

    // Extract the shares
    let s_i_cj = registration.forward_verification_shares.to_bigint(); // S_i,Cj
    let s_i_cj_prime = registration.reverse_verification_shares.to_bigint(); // S_i,Cj'
    let stild_i_cj = registration.forward_ballot_shares.to_bigint(); // S~i,Cj
    let stild_i_cj_prime = registration.reverse_ballot_shares.to_bigint(); // S~i,Cj'

    Self {
      // Collector details (Don't have these right now)
      collector_index: 0,      // Initialized later
      num_collectors: 0,       // Initialized later
      public_keys: Vec::new(), // Initialized later

      // Election Parameters
      generator,
      prime,
      num_registered,
      num_candidates: question.num_candidates,

      // Ballots  (Don't have this right now)
      p_i: BigInt::from(0),       // Initialized later
      p_i_prime: BigInt::from(0), // Initialized later

      // Commitments (Also don't have this right now)
      g_s: BigInt::from(0),         // Initialized later
      g_s_prime: BigInt::from(0),   // Initialized later
      g_s_s_prime: BigInt::from(0), // Initialized later

      // Private key
      n: &paillier_p * &paillier_q,
      paillier_p,
      paillier_q,
      rsa_a,
      rsa_b,

      // Shares held by the collector
      s_i_cj,           // S_i,Cj
      s_i_cj_prime,     // S_i,Cj'
      stild_i_cj,       // S~i,Cj
      stild_i_cj_prime, // S~i,Cj'

      // Values for sub-protocol 1 (Don't have right now)
      sp1_values: BTreeMap::new(), // Initialized later
      products: BTreeMap::new(),   // Initialized later

      // Values for sub-protocol 2 (Don't have right now)
      sp2_values: BTreeMap::new(), // Initialized later
    }
  }

  /// Verify the signature on a message using the internal public key
  ///
  /// Sends a error close message if the signature validation fails
  fn verify_signature<T: SignedMessage>(&self, msg: &T, ctx: &mut <Self as Actor>::Context) -> bool {
    let self_addr = ctx.address();
    match self.public_keys.get(msg.get_from()) {
      Some(public_key) if msg.verify_signature(public_key) => true,
      Some(_) => {
        self_addr.do_send(ErrorClose::from((
          CloseCode::Invalid,
          format!("Invalid signature from collector {}", msg.get_from() + 1),
        )));
        false
      }

      None => {
        self_addr.do_send(ErrorClose::from((
          CloseCode::Abnormal,
          format!(
            "Cannot verify signature, public key for collector {} is not known",
            msg.get_from() + 1
          ),
        )));
        false
      }
    }
  }

  /// Send an unsigned message to the mediator
  fn send_mediator_unsigned<T: Serialize>(data: T, ctx: &mut <Self as Actor>::Context) {
    Self::send_json(&MediatorMessage { data }, ctx)
  }

  /// Send a signed message to the mediator
  fn send_mediator<T: Serialize + Hash>(&self, data: T, ctx: &mut <Self as Actor>::Context) {
    Self::send_json(
      &SignedMediatorMessage::new_signed(self.collector_index, data, &self.rsa_a, &self.n),
      ctx,
    )
  }

  /// Signed a signed message to another collector
  fn send_unicast<T: Serialize + Hash>(&self, to: usize, data: T, ctx: &mut <Self as Actor>::Context) {
    Self::send_json(
      &SignedUnicastMessage::new_signed(self.collector_index, to, data, &self.rsa_a, &self.n),
      ctx,
    )
  }

  /// Signed a signed message to all collectors
  fn send_broadcast<T: Serialize + Hash>(&self, data: T, ctx: &mut <Self as Actor>::Context) {
    Self::send_json(
      &SignedBroadcastMessage::new_signed(self.collector_index, data, &self.rsa_a, &self.n),
      ctx,
    )
  }

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
impl Actor for VerificationWebsocketActor {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    // Broadcast the public key back to the mediator
    Self::send_mediator_unsigned(
      PublicKey {
        n: self.n.clone(),
        b: self.rsa_b.clone(),
      },
      ctx,
    );
  }
}

///
/// Handler for individual websocket messages
///
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for VerificationWebsocketActor {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    let self_addr = ctx.address();

    log::debug!("Received message: {:#?}", msg);
    let msg: ws::Message = match msg {
      Err(e) => return self_addr.do_send(ErrorClose::from((CloseCode::Error, format!("{}", e)))),
      Ok(msg) => msg,
    };

    // Parse as a JSON string
    let json: WebsocketMessage = match msg {
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
      ws::Message::Text(text) => match serde_json::from_str::<WebsocketMessage>(&text) {
        Err(e) => return self_addr.do_send(ErrorClose::from((CloseCode::Invalid, format!("Invalid JSON: {}", e)))),
        Ok(json) => json,
      },

      // Unsupported messages
      ws::Message::Binary(_) => return self_addr.do_send(ErrorClose::from((CloseCode::Unsupported, "Binary Data"))),
      ws::Message::Continuation(_) => {
        return self_addr.do_send(ErrorClose::from((CloseCode::Unsupported, "Continuation Frame")))
      }
    };

    // Handle all of the messages, verifying signature first if necessary
    match json {
      WebsocketMessage::Initialize(data) => self_addr.do_send(data),
      WebsocketMessage::SP1_STMP_Request(data) => {
        if self.verify_signature(&data, ctx) {
          self_addr.do_send(data);
        }
      }
      WebsocketMessage::SP1_STMP_Response(data) => {
        if self.verify_signature(&data, ctx) {
          self_addr.do_send(data);
        }
      }
      WebsocketMessage::SP1_Product_Response(data) => {
        if self.verify_signature(&data, ctx) {
          self_addr.do_send(data);
        }
      }
      WebsocketMessage::SP2_Shares_Response(data) => {
        if self.verify_signature(&data, ctx) {
          self_addr.do_send(data);
        }
      }
    }
  }

  fn finished(&mut self, ctx: &mut Self::Context) {
    log::info!("Websocket stream closed, stopping actor");
    ctx.stop()
  }
}

///
/// Close the websocket due to an error
///
impl Handler<ErrorClose> for VerificationWebsocketActor {
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
/// Handle the "Initialize" message from the mediator
///
impl Handler<MediatorMessage<Initialize>> for VerificationWebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: MediatorMessage<Initialize>, ctx: &mut Self::Context) -> Self::Result {
    let init = msg.data;
    log::debug!("Initialize parameters for collector {}:", init.collector_index + 1);

    // Collector details
    self.collector_index = init.collector_index;
    self.num_collectors = init.num_collectors;
    self.public_keys = init.public_keys;

    // Ballots
    self.p_i = init.forward_ballot;
    self.p_i_prime = init.reverse_ballot;

    // Commitments
    self.g_s = init.g_s;
    self.g_s_prime = init.g_s_prime;
    self.g_s_s_prime = init.g_s_s_prime;

    // ===================================
    // Send out the sub-protocol 1 STPM requests
    //  The requests are summetric, so only send to the upper triangle half
    // ===================================
    if (self.collector_index + 1) < self.num_collectors {
      log::debug!(
        "Sub-protocol 1 - Send STPM requests to collectors {} to {}",
        self.collector_index + 2,
        self.num_collectors
      );
    }

    for index in (self.collector_index + 1)..self.num_collectors {
      let data = SP1_STMP_Request {
        e_s_cj: stpm::step_1(&self.s_i_cj, &self.n),
        e_s_cj_prime: stpm::step_1(&self.s_i_cj_prime, &self.n),
      };

      self.send_unicast(index, data, ctx);
      log::debug!("Sent parameters to collector {}", index + 1);
    }

    // ===================================
    // Compute values for sub-protocol 2
    //   g^(S~i,Cj) and g^(S~i,Cj')
    // ===================================
    log::debug!("Sub-protocol 2 - Compute g values");
    let g_stild = BigInt::mod_pow(&self.generator, &self.stild_i_cj, &self.prime); // g^(S~i,Cj)
    let g_stild_prime = BigInt::mod_pow(&self.generator, &self.stild_i_cj_prime, &self.prime); // g^(S~i,Cj')

    log::debug!("g^(S~i,C{}) = {}", self.collector_index + 1, g_stild);
    log::debug!("g^(S~i,C{}') = {}", self.collector_index + 1, g_stild_prime);

    // Broadcast the sub-protocol 2 values to all websockets
    let sp2_value = SP2_Shares_Response { g_stild, g_stild_prime };
    self.send_broadcast(&sp2_value, ctx);

    // Then save the broadcasted values into current user database
    self.sp2_values.insert(self.collector_index, sp2_value);
  }
}

///
/// Handle the "STPM Request" message from a specific collector
///
impl Handler<SignedUnicastMessage<SP1_STMP_Request>> for VerificationWebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: SignedUnicastMessage<SP1_STMP_Request>, ctx: &mut Self::Context) -> Self::Result {
    // ============================================
    //   First STPM: rj + rk' = S_i,Cj * S_i,Ck'
    // ============================================
    log::debug!(
      "Sub-protocol 1: First STPM: r{0} + r{1}' = S_i,C{0} * S_i,C{1}'",
      self.collector_index + 1,
      msg.from + 1
    );

    let (r_prime, e_s_cj_e_rk_prime) = stpm::step_2(&msg.data.e_s_cj, &self.s_i_cj_prime, &self.n, true);
    log::debug!("r{}' = {}", msg.from + 1, r_prime);

    // ============================================
    //   Second STPM: rj' + rk = S_i,Cj' * S_i,Ck
    // ============================================
    log::debug!(
      "Sub-protocol 1: Second STPM: r{0}' + r{1} = S_i,C{0}' * S_i,C{1}",
      self.collector_index + 1,
      msg.from + 1
    );

    let (r, e_s_cj_prime_e_rk) = stpm::step_2(&msg.data.e_s_cj_prime, &self.s_i_cj, &self.n, true);
    log::debug!("r{} = {}", msg.from + 1, r);

    // Set these values inside the map
    self.sp1_values.insert(msg.from, SP1_STPM_Values { r, r_prime });

    // Return the response back to the user
    self.send_unicast(
      msg.from,
      SP1_STMP_Response {
        e_s_cj_e_rk_prime,
        e_s_cj_prime_e_rk,
      },
      ctx,
    );
  }
}

///
/// Handle the "STPM Response" message from a specific collector
///
impl Handler<SignedUnicastMessage<SP1_STMP_Response>> for VerificationWebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: SignedUnicastMessage<SP1_STMP_Response>, ctx: &mut Self::Context) -> Self::Result {
    // ============================================
    //   First STPM: rj + rk' = S_i,Cj * S_i,Ck'
    // ============================================
    log::debug!(
      "Sub-protocol 1: First STPM: r{0} + r{1}' = S_i,C{0} * S_i,C{1}'",
      self.collector_index + 1,
      msg.from + 1
    );

    let r = stpm::step_3(&msg.data.e_s_cj_e_rk_prime, &self.paillier_p, &self.paillier_q, true);
    log::debug!("r{} = {}", self.collector_index + 1, r);

    // ============================================
    //   Second STPM: rj' + rk = S_i,Cj' * S_i,Ck
    // ============================================
    log::debug!(
      "Sub-protocol 1: Second STPM: r{0}' + r{1} = S_i,C{0}' * S_i,C{1}",
      self.collector_index + 1,
      msg.from + 1
    );

    let r_prime = stpm::step_3(&msg.data.e_s_cj_prime_e_rk, &self.paillier_p, &self.paillier_q, true);
    log::debug!("r{}' = {}", self.collector_index + 1, r_prime);

    // Set these values inside the map
    self.sp1_values.insert(msg.from, SP1_STPM_Values { r, r_prime });

    // Possibly compute the combined product
    self.maybe_publish_combined_product(ctx);
  }
}

impl VerificationWebsocketActor {
  /// Publish the combined product if we have all values from STPM
  fn maybe_publish_combined_product(&mut self, ctx: &mut <Self as Actor>::Context) {
    if (self.sp1_values.len() + 1) != self.num_collectors {
      return; // Don't have all products yet
    }

    log::debug!(
      "Sub-protocol 1: Computing combined product P{}",
      self.collector_index + 1
    );

    // Add together all r values from STPM
    let modulus = &self.prime - 1;
    let r_r_prime = self.sp1_values.values().fold(BigInt::from(0), |acc, value| {
      BigInt::mod_add(&acc, &(&value.r + &value.r_prime), &modulus)
    });

    // Now compute the product
    let product_j = BigInt::mod_mul(
      &BigInt::mod_mul(
        &BigInt::mod_pow(&self.g_s, &self.s_i_cj_prime, &self.prime),
        &BigInt::mod_pow(&self.g_s_prime, &self.s_i_cj, &self.prime),
        &self.prime,
      ),
      &BigInt::mod_mul(
        &BigInt::mod_pow(&self.generator, &(&self.s_i_cj * &self.s_i_cj_prime), &self.prime),
        &BigInt::mod_pow(&self.generator, &r_r_prime, &self.prime),
        &self.prime,
      ),
      &self.prime,
    );

    log::debug!("P{} = {}", self.collector_index + 1, product_j);

    // Broadcaast the sub-protocol 1 product to all collectors
    let product_response = SP1_Product_Response { product_j };
    self.send_broadcast(&product_response, ctx);

    // Also set the product inside the map
    self.products.insert(self.collector_index, product_response.product_j);

    // Possibly do the final verification if we have all the products
    self.maybe_publish_sp1_result(ctx);
  }
}

///
/// Handle the sub-protocol 1 product broadcast message
///
impl Handler<SignedBroadcastMessage<SP1_Product_Response>> for VerificationWebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: SignedBroadcastMessage<SP1_Product_Response>, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Sub-protocol 1 - Received product from collector {}", msg.from + 1);

    // Insert into the map of products
    log::debug!("P{} = {}", msg.from + 1, msg.data.product_j);
    self.products.insert(msg.from, msg.data.product_j);

    // Possibly do the final verification if we have all the products
    self.maybe_publish_sp1_result(ctx);
  }
}

impl VerificationWebsocketActor {
  /// Publish the final result from sub-protocol 1 if we have all values needed
  fn maybe_publish_sp1_result(&mut self, ctx: &mut <Self as Actor>::Context) {
    if self.products.len() != self.num_collectors {
      return; // Don't have all products yet
    }

    log::debug!("Sub-protocol 1: Computing final combined product");

    // Multiply all products from all collectors
    let products = self
      .products
      .values()
      .fold(BigInt::from(0), |acc, x| BigInt::mod_mul(&acc, &x, &self.prime));

    log::debug!("P1 * ... * P{} = {}", self.num_collectors, products);

    // Compute the final combined product
    let combined_product = BigInt::mod_mul(&self.g_s_s_prime, &products, &self.prime);
    log::debug!(
      "g^(s_i * s_i') * P1 * ... * P{} = {}",
      self.num_collectors,
      combined_product
    );

    // Compute the expected product:
    //   g^(2^(L - 1)), where L is the number of bits in the voting vector
    let expected_product = BigInt::mod_pow(
      &self.generator,
      &(BigInt::from(1) << (self.num_registered * self.num_candidates - 1) as usize),
      &self.prime,
    );
    log::debug!("g^(2^(L - 1)) = {}", expected_product);

    // Send the verification result
    let sp1_ballot_valid = combined_product == expected_product;
    log::debug!(
      "Sub-protocol 1: ballot {}",
      if sp1_ballot_valid { "valid" } else { "invalid" }
    );

    // Send the final result to the mediator
    self.send_mediator(SP1_Result_Response { sp1_ballot_valid }, ctx);
  }
}

///
/// Handle the sub-protocol 2 shares broadcast
///
impl Handler<SignedBroadcastMessage<SP2_Shares_Response>> for VerificationWebsocketActor {
  type Result = ();

  fn handle(&mut self, msg: SignedBroadcastMessage<SP2_Shares_Response>, ctx: &mut Self::Context) -> Self::Result {
    log::debug!("Sub-protocol 2 - Received g values from collector {}", msg.from + 1);

    // Insert into the map of values
    log::debug!("g^(S~i,C{}) = {}", msg.from + 1, msg.data.g_stild);
    log::debug!("g^(S~i,C{}') = {}", msg.from + 1, msg.data.g_stild_prime);
    self.sp2_values.insert(msg.from, msg.data);

    // Also publish the final result if all values are available
    self.maybe_publish_sp2_result(ctx);
  }
}

impl VerificationWebsocketActor {
  /// Publish the final result from sub-protocol 2 if we have all the values needed
  fn maybe_publish_sp2_result(&mut self, ctx: &mut <Self as Actor>::Context) {
    if self.sp2_values.len() != self.num_collectors {
      return; // Don't have all g^(shares) yet
    }

    log::debug!("Sub-protocol 2 - Compute the final products");

    // Multiply all g products together
    let (g_combined, g_prime_combined) =
      self
        .sp2_values
        .values()
        .fold((BigInt::from(0), BigInt::from(0)), |(forward, reverse), value| {
          (
            BigInt::mod_mul(&forward, &value.g_stild, &self.prime),
            BigInt::mod_mul(&reverse, &value.g_stild_prime, &self.prime),
          )
        });

    log::debug!("g^(S~i,C1) * ... * g^(S~i,C{}) = {}", self.num_collectors, g_combined);
    log::debug!(
      "g^(S~i,C1') * ... * g^(S~i,C{}') = {}",
      self.num_collectors,
      g_prime_combined
    );

    // Verify the forward ballot
    let g_p_i = BigInt::mod_pow(&self.generator, &self.p_i, &self.prime);
    let g_p_i_combined = BigInt::mod_mul(&self.g_s, &g_combined, &self.prime);
    let g_p_i_verified = &g_p_i_combined == &g_p_i;

    log::debug!("Sub-protocol 2: Forward ballot");
    log::debug!("g^(p_i) = {}", g_p_i);
    log::debug!(
      "g^(s_i) * g^(S~i,C1) * ... * g^(S~i,C{}) = {}",
      self.num_collectors,
      g_p_i_combined
    );
    log::debug!("Forward ballot {}", if g_p_i_verified { "valid" } else { "invalid" });

    // Verify the reverse ballot
    let g_p_i_prime = BigInt::mod_pow(&self.generator, &self.p_i_prime, &self.prime);
    let g_p_i_prime_combined = BigInt::mod_mul(&self.g_s_prime, &g_prime_combined, &self.prime);
    let g_p_i_prime_verified = &g_p_i_prime_combined == &g_p_i_prime;

    log::debug!("Sub-protocol 2: Reverse ballot");
    log::debug!("g^(p_i') = {}", g_p_i_prime);
    log::debug!(
      "g^(s_i') * g^(S~i,C1') * ... * g^(S~i,C{}') = {}",
      self.num_collectors,
      g_p_i_prime_combined
    );
    log::debug!(
      "Reverse ballot {}",
      if g_p_i_prime_verified { "valid" } else { "invalid" }
    );

    // Final verification results
    let sp2_ballot_valid = g_p_i_verified && g_p_i_prime_verified;
    log::debug!(
      "Sub-protocol 2: ballot {}",
      if sp2_ballot_valid { "valid" } else { "invalid" }
    );

    // Send the final result to the mediator
    self.send_mediator(SP2_Result_Response { sp2_ballot_valid }, ctx);
  }
}
