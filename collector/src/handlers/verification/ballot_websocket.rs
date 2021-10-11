use actix::prelude::*;
use actix_http::ws::{CloseCode, CloseReason};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use curv_kzen::arithmetic::{BitManipulation, Modulo, Samplable};
use curv_kzen::BigInt;
use kzen_paillier::*;
use serde::{Deserialize, Serialize};

use super::{Product1, Product2, VerificationResult, VerifyBallotWebsocketData, VerifyBallotWebsocketResult};
use crate::db::DbConnection;
use crate::models::{Election, Registration};
use crate::utils::ConvertBigInt;

/// Structure used for managing the websocket protocol
///
/// This protocol verifies both sub-protocol 1 and sub-protocol 2 over websockets.
/// Messages are combined to limit the amount of back-and-forth communication.
///
/// The protocol is symmetric as long as we keep track of the "opposite" collector.
/// To make the convention easier, we assume the client is C1 and the server actor is C2
pub struct BallotWebsocket {
  // Database connection
  conn: DbConnection,

  // Published ballots
  forward_ballot: BigInt,
  reverse_ballot: BigInt,

  // Commitments
  g_s: BigInt,
  g_s_prime: BigInt,
  g_s_s_prime: BigInt,

  // Shares held by the collector
  s_i_c2: BigInt,           // S_i,C2
  s_i_c2_prime: BigInt,     // S_i,C2'
  stild_i_c2: BigInt,       // S~i,C2
  stild_i_c2_prime: BigInt, // S~i,C2'

  // g^x (mod p) is a cyclic group of order p-1
  generator: BigInt,
  prime: BigInt,

  // Modulus for Paillier encryption
  n: BigInt,

  // Exchanges S_i,C1 * S_i,C2'
  e_s_c1: BigInt, // E(S_i,C1, e)
  r2_prime: BigInt,

  // Exchanges S_i,C1' * S_i,C2
  e_s_c1_prime: BigInt, // E(S_i,C1', e)
  r2: BigInt,

  // Combined products
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

impl BallotWebsocket {
  pub fn new(
    data: VerifyBallotWebsocketData,
    election: Election,
    registration: Registration,
    conn: DbConnection,
  ) -> Self {
    let generator = election.generator.to_bigint();
    let prime = election.prime.to_bigint();

    // Pick values for r2' and r2 for STPM
    let r2_prime = BigInt::sample_below(&prime);
    let r2 = BigInt::sample_below(&prime);

    // Extract the shares
    let s_i_c2 = registration.forward_verification_shares.to_bigint(); // S_i,C2
    let s_i_c2_prime = registration.reverse_verification_shares.to_bigint(); // S_i,C2'
    let stild_i_c2 = registration.forward_ballot_shares.to_bigint(); // S~i,C2
    let stild_i_c2_prime = registration.reverse_ballot_shares.to_bigint(); // S~i,C2'

    // Compute values for sub-protocol 2
    let g_stild_2 = BigInt::mod_pow(&generator, &stild_i_c2, &prime); // g^(S~i,C2)
    let g_stild_2_prime = BigInt::mod_pow(&generator, &stild_i_c2_prime, &prime); // g^(S~i,C2')

    let g_p_i = BigInt::mod_pow(&generator, &data.forward_ballot, &prime);
    let g_p_i_prime = BigInt::mod_pow(&generator, &data.reverse_ballot, &prime);

    Self {
      conn,

      // Ballots
      forward_ballot: data.forward_ballot,
      reverse_ballot: data.reverse_ballot,

      // Commitments
      g_s: data.g_s,
      g_s_prime: data.g_s_prime,
      g_s_s_prime: data.g_s_s_prime,

      // Shares held by the collector
      s_i_c2,           // S_i,C2
      s_i_c2_prime,     // S_i,C2'
      stild_i_c2,       // S~i,C2
      stild_i_c2_prime, // S~i,C2'

      // g^x (mod p) is a cyclic group of order p-1
      generator,
      prime,

      // Modulus for Paillier encryption
      n: data.n,

      // Exchanges S_i,C1 * S_i,C2'
      e_s_c1: data.e_s_c1, // E(S_i,C1, e)
      r2_prime,

      // Exchanges S_i,C1' * S_i,C2
      e_s_c1_prime: data.e_s_c1_prime, // E(S_i,C1', e)
      r2,

      // Combined products (Compute later)
      p1: BigInt::from(0),
      p2: BigInt::from(0),

      // Sub-protocol 2 values
      g_stild_1: data.g_stild_1,             // g^(S~i,C1)
      g_stild_1_prime: data.g_stild_1_prime, // g^(S~i,C1')
      g_stild_2,                             // g^(S~i,C2)
      g_stild_2_prime,                       // g^(S~i,C2')

      g_p_i,       // g^(p_i)
      g_p_i_prime, // g^(p_i')
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
enum AllClientMessages {
  Product1(Product1),
}

impl Actor for BallotWebsocket {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    let ek: EncryptionKey = MinimalEncryptionKey { n: self.n.clone() }.into();

    // Compute E(r2', e)
    let encrypt_r2_prime: RawCiphertext = Paillier::encrypt(&ek, self.r2_prime.clone().into());

    // Compute (E(S_i,C1, e)^(S_i,C2')) * (E(r2', e)^(-1)) (mod n^2)
    let e_r1 = BigInt::mod_mul(
      &BigInt::mod_pow(&self.e_s_c1, &self.s_i_c2_prime, &ek.nn),
      &BigInt::mod_inv(&encrypt_r2_prime.0, &ek.nn).expect("Error: No Inverse"),
      &ek.nn,
    );

    // Compute E(r2, e)
    let encrypt_r2: RawCiphertext = Paillier::encrypt(&ek, self.r2.clone().into());

    // Compute (E(S_i,C1', e)^(S_i,C2)) * (E(r2, e)^(-1)) (mod n^2)
    let e_r1_prime = BigInt::mod_mul(
      &BigInt::mod_pow(&self.e_s_c1_prime, &self.s_i_c2, &ek.nn),
      &BigInt::mod_inv(&encrypt_r2.0, &ek.nn).expect("Error: No Inverse"),
      &ek.nn,
    );

    // Values to send back
    let message = VerifyBallotWebsocketResult {
      e_s_c1_e_r2_prime: e_r1,
      e_s_c1_prime_r2: e_r1_prime,
      g_stild_2: self.g_stild_2.clone(),
      g_stild_2_prime: self.g_stild_2_prime.clone(),
    };

    // TODO:
    ctx.text(serde_json::to_string(&message).unwrap());
  }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for BallotWebsocket {
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
      AllClientMessages::Product1(data) => self_addr.do_send(data),
    }
  }
}

///
/// Close the websocket due to an error
///
impl Handler<ErrorClose> for BallotWebsocket {
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
/// Handle product 1 response
///
impl Handler<Product1> for BallotWebsocket {
  type Result = ();

  fn handle(&mut self, product1: Product1, ctx: &mut Self::Context) -> Self::Result {
    self.p1 = product1.p1;

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

    let message = Product2 { p2 };

    // TODO:
    ctx.text(serde_json::to_string(&message).unwrap());
  }
}
