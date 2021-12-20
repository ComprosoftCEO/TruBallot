#![allow(non_camel_case_types)]
use actix::prelude::*;
use actix_http::ws::CloseCode;
use curv_kzen::arithmetic::Modulo;
use curv_kzen::BigInt;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use super::sha_hasher::SHAHasher;

/// Close the mediator actor due to an error
#[derive(Message)]
#[rtype(result = "()")]
pub struct ErrorClose(pub CloseCode, pub Option<String>);

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

/// Top level structure for any type of JSON value that can be received by the mediator
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WebsocketMessage {
  PublicKey(PublicKey),
  SP1_Result_Response(SignedMediatorMessage<SP1_Result_Response>),
  SP2_Result_Response(SignedMediatorMessage<SP2_Result_Response>),

  // Forward these messages without any signature verification
  // (Does check to make sure it didn't lie about "from" field)
  UnicastMessage(SignedUnicastMessage),
  BroadcastMessage(SignedBroadcastMessage),
}

/// Messages that have a "from" field
pub trait OriginMessage {
  /// Extract the source of the message
  fn get_from(&self) -> usize;
}

/// Represents a received message that is signed
pub trait SignedMessage: OriginMessage {
  /// Extract the signature stored in the struct
  fn get_signature(&self) -> &BigInt;

  /// Hash the message to get the signature
  fn compute_hash(&self) -> BigInt;

  /// Verify the message signature using RSA from the public key
  fn verify_signature(&self, public_key: &PublicKey) -> bool {
    self.get_signature() == &BigInt::mod_pow(&self.compute_hash(), &public_key.b, &public_key.n)
  }
}

///
/// Received an unsigned message for the mediator
///
#[derive(Debug, Clone, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct UnsignedMediatorMessage<T> {
  pub from: usize,
  pub data: T,
}

impl<T> OriginMessage for UnsignedMediatorMessage<T> {
  #[inline]
  fn get_from(&self) -> usize {
    self.from
  }
}

///
/// Received a signed message for the mediator
///
#[derive(Debug, Clone, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SignedMediatorMessage<T> {
  pub from: usize,
  pub data: T,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub collector_signature: BigInt,
}

impl<T> OriginMessage for SignedMediatorMessage<T> {
  #[inline]
  fn get_from(&self) -> usize {
    self.from
  }
}

impl<T: Hash> SignedMessage for SignedMediatorMessage<T> {
  #[inline]
  fn get_signature(&self) -> &BigInt {
    &self.collector_signature
  }

  fn compute_hash(&self) -> BigInt {
    let mut hasher = SHAHasher::new();
    self.data.hash(&mut hasher);
    hasher.get_sha_hash()
  }
}

///
/// Message to send to or receive from a specific websocket
///  This is simply forwarded verbatum, uses serde_json::Value
///  to accept any valid type
///
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SignedUnicastMessage {
  pub from: usize,
  pub to: usize,
  pub data: serde_json::Value,

  /// RSA Signature
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub signature: BigInt,
}

impl OriginMessage for SignedUnicastMessage {
  #[inline]
  fn get_from(&self) -> usize {
    self.from
  }
}

///
/// Message to send to or receive from ALL websockets
///  This is simply forwarded verbatum, uses serde_json::Value
///  to accept any valid type
///
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SignedBroadcastMessage {
  pub from: usize,
  pub data: serde_json::Value,

  /// RSA Signature
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub signature: BigInt,
}

impl OriginMessage for SignedBroadcastMessage {
  #[inline]
  fn get_from(&self) -> usize {
    self.from
  }
}

// =============================================
// Define all data structures from the mediator
// =============================================

/// Publish the public key for a collector
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
  /// Modulus for both RSA and Paillier cryptosystem
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub n: BigInt,

  /// Public exponent b for RSA
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub b: BigInt,

  /// Signature to ensure public key has been faithfully published
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub signature: BigInt,
}

/// Initialization parameters to send to the websocket
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Initialize {
  // Collector details
  pub collector_index: usize,
  pub num_collectors: usize,

  // Ballots
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_ballot: BigInt, // p_i
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_ballot: BigInt, // p_i'

  // Commitments
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s: BigInt, // g^(s_i)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_prime: BigInt, // g^(s_i')
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_s_prime: BigInt, // g^(s_i * s_i')

  // STPM Encryption Key and RSA signatures
  pub public_keys: Vec<PublicKey>,
}

// =============================================
// Define all data structures for Sub-Protocols
// =============================================

/// Sub-Protocol 1 - Final result from the websocket
#[derive(Debug, Clone, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_Result_Response {
  pub sp1_ballot_valid: bool,
}

/// Sub-Protocol 2 - Final result from the websocket
#[derive(Debug, Clone, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP2_Result_Response {
  pub sp2_ballot_valid: bool,
}
