#![allow(non_camel_case_types)]
use actix::prelude::*;
use actix_http::ws::CloseCode;
use curv_kzen::arithmetic::{Converter, Modulo};
use curv_kzen::BigInt;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use super::sha_hasher::SHAHasher;
use super::verification_websocket_actor::VerificationWebsocketActor;

/// Close the JSON connection due to an error
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

/// Top level structure for any type of JSON value that can be received
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WebsocketMessage {
  MediatorMessage(MediatorMessage<AllMediatorMessages>),
  UnicastMessage(SignedUnicastMessage<AllUnicastMessages>),
  BroadcastMessage(SignedBroadcastMessage<AllBroadcastMessages>),
}

impl WebsocketMessage {
  /// Check if a message has a signature and get the "from" field
  pub fn get_from(&self) -> Option<usize> {
    match self {
      WebsocketMessage::MediatorMessage(_) => None,
      WebsocketMessage::UnicastMessage(message) => Some(message.get_from()),
      WebsocketMessage::BroadcastMessage(message) => Some(message.get_from()),
    }
  }

  /// Verify the signature on a message if it is signed
  pub fn verify_signature_if_signed(&self, public_key: &PublicKey) -> bool {
    match self {
      WebsocketMessage::MediatorMessage(_) => true,
      WebsocketMessage::UnicastMessage(message) => message.verify_signature(public_key),
      WebsocketMessage::BroadcastMessage(message) => message.verify_signature(public_key),
    }
  }

  /// Send the internal message to the actor, NOT checking the signature
  pub fn send_actor_message_ignore_signature(self, addr: Addr<VerificationWebsocketActor>) {
    match self {
      WebsocketMessage::MediatorMessage(MediatorMessage { data }) => match data {
        AllMediatorMessages::Initialize(data) => addr.do_send(MediatorMessage { data }),
      },

      WebsocketMessage::UnicastMessage(SignedUnicastMessage {
        from,
        to,
        data,
        signature,
      }) => match data {
        AllUnicastMessages::SP1_STMP_Request(data) => addr.do_send(SignedUnicastMessage {
          from,
          to,
          data,
          signature,
        }),
        AllUnicastMessages::SP1_STMP_Response(data) => addr.do_send(SignedUnicastMessage {
          from,
          to,
          data,
          signature,
        }),
      },

      WebsocketMessage::BroadcastMessage(SignedBroadcastMessage { from, data, signature }) => match data {
        AllBroadcastMessages::SP1_Product_Response(data) => {
          addr.do_send(SignedBroadcastMessage { from, data, signature })
        }
        AllBroadcastMessages::SP2_Shares_Response(data) => {
          addr.do_send(SignedBroadcastMessage { from, data, signature })
        }
      },
    }
  }
}

/// All messages that can be RECEIVED from a mediator
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AllMediatorMessages {
  Initialize(Initialize),
}

/// All messages that can be RECEIVED from a specific websocket (Unicast)
#[derive(Debug, Clone, Hash, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum AllUnicastMessages {
  SP1_STMP_Request(SP1_STMP_Request),
  SP1_STMP_Response(SP1_STMP_Response),
}

/// All messages that can be RECEIVED from any websocket
#[derive(Debug, Clone, Hash, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum AllBroadcastMessages {
  SP1_Product_Response(SP1_Product_Response),
  SP2_Shares_Response(SP2_Shares_Response),
}

/// Represents a received message that is signed
pub trait SignedMessage {
  /// Extract the signature stored in the struct
  fn get_signature(&self) -> &BigInt;

  /// Extract the source of the signature
  fn get_from(&self) -> usize;

  /// Hash the message to get the signature
  fn compute_hash(&self) -> BigInt;

  /// Verify the message signature using RSA from the public key
  fn verify_signature(&self, public_key: &PublicKey) -> bool {
    self.get_signature() == &BigInt::mod_pow(&self.compute_hash(), &public_key.b, &public_key.n)
  }
}

///
/// Unsigned message received from a mediator
///
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct MediatorMessage<T> {
  pub data: T,
}

///
/// Send a signed message to the mediator
///
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedMediatorMessage<T> {
  pub from: usize,
  pub data: T,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub collector_signature: BigInt,
}

impl<T: Hash> SignedMediatorMessage<T> {
  pub fn new_signed(from: usize, data: T, private_key: &BigInt, n: &BigInt) -> Self {
    // Start by building the message
    let mut message = Self {
      from,
      data,
      collector_signature: BigInt::from(0),
    };

    // Then compute the signature from the hash
    message.collector_signature = BigInt::mod_pow(&message.compute_hash(), private_key, n);
    message
  }
}

impl<T: Hash> SignedMessage for SignedMediatorMessage<T> {
  fn get_signature(&self) -> &BigInt {
    &self.collector_signature
  }

  fn get_from(&self) -> usize {
    self.from
  }

  fn compute_hash(&self) -> BigInt {
    let mut hasher = SHAHasher::new();
    self.data.hash(&mut hasher);
    hasher.get_sha_hash()
  }
}

///
/// Message to send to or receive from a specific websocket
///
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SignedUnicastMessage<T> {
  pub from: usize,
  pub to: usize,
  pub data: T,

  /// RSA Signature
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub signature: BigInt,
}

impl<T: Hash> SignedUnicastMessage<T> {
  pub fn new_signed(from: usize, to: usize, data: T, private_key: &BigInt, n: &BigInt) -> Self {
    // Start by building the message
    let mut message = Self {
      from,
      to,
      data,
      signature: BigInt::from(0),
    };

    // Then compute the signature from the hash
    message.signature = BigInt::mod_pow(&message.compute_hash(), private_key, n);
    message
  }
}

impl<T: Hash> SignedMessage for SignedUnicastMessage<T> {
  fn get_signature(&self) -> &BigInt {
    &self.signature
  }

  fn get_from(&self) -> usize {
    self.from
  }

  fn compute_hash(&self) -> BigInt {
    let mut hasher = SHAHasher::new();
    self.from.hash(&mut hasher);
    self.to.hash(&mut hasher);
    self.data.hash(&mut hasher);
    hasher.get_sha_hash()
  }
}

///
/// Message to send to or receive from ALL websockets
///
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SignedBroadcastMessage<T> {
  pub from: usize,
  pub data: T,

  /// RSA Signature
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub signature: BigInt,
}

impl<T: Hash> SignedBroadcastMessage<T> {
  pub fn new_signed(from: usize, data: T, private_key: &BigInt, n: &BigInt) -> Self {
    // Start by building the message
    let mut message = Self {
      from,
      data,
      signature: BigInt::from(0),
    };

    // Then compute the signature from the hash
    message.signature = BigInt::mod_pow(&message.compute_hash(), private_key, n);
    message
  }
}

impl<T: Hash> SignedMessage for SignedBroadcastMessage<T> {
  fn get_signature(&self) -> &BigInt {
    &self.signature
  }

  fn get_from(&self) -> usize {
    self.from
  }

  fn compute_hash(&self) -> BigInt {
    let mut hasher = SHAHasher::new();
    self.from.hash(&mut hasher);
    self.data.hash(&mut hasher);
    hasher.get_sha_hash()
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
}

/// Initialization parameters to send to the websocket
#[derive(Debug, Clone, Deserialize)]
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
// Define all data structures for Sub-Protocol 1
// =============================================

/// Sub-Protocol 1 - Secure Two-Party Multiplication Request
///
/// This facilitates STPM with collector j and collector k
///
/// Computes:
///   rj + rk' = S_i,Cj * S_i,Ck'
///   rj' + rk = S_i,Cj' * S_i,Ck
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_STMP_Request {
  // E(S_i,Cj, e)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_cj: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_cj_prime: BigInt, // E(S_i,Cj', e)
}

impl Hash for SP1_STMP_Request {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.e_s_cj.to_bytes().hash(state);
    self.e_s_cj_prime.to_bytes().hash(state);
  }
}

/// Sub-Protocol 1 - Secure Two-Party Multiplication Response
///
/// This facilitates STPM with collector j and collector k
///
/// Computes:
///   rj + rk' = S_i,Cj * S_i,Ck'
///   rj' + rk = S_i,Cj' * S_i,Ck
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_STMP_Response {
  // (E(S_i,Cj, e)^(S_i,Ck')) * (E(rk', e)^(-1)) (mod n^2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_cj_e_rk_prime: BigInt,

  // (E(S_i,Cj', e)^(S_i,Ck)) * (E(rk, e)^(-1)) (mod n^2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_cj_prime_e_rk: BigInt,
}

impl Hash for SP1_STMP_Response {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.e_s_cj_e_rk_prime.to_bytes().hash(state);
    self.e_s_cj_prime_e_rk.to_bytes().hash(state);
  }
}

/// Sub-Protocol 1 - Computed product response for collector j
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_Product_Response {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub product_j: BigInt,
}

impl Hash for SP1_Product_Response {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.product_j.to_bytes().hash(state);
  }
}

/// Sub-Protocol 1 - Final result from the websocket
#[derive(Debug, Clone, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_Result_Response {
  pub ballot_valid: bool,
}

// =============================================
// Define all data structures for Sub-Protocol 2
// =============================================

/// Sub-Protocol 2 - Computed values g^(S~i,Cj) and g^(S~i,Cj') for collector j
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP2_Shares_Response {
  // g^(S~i,Cj)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild: BigInt,

  // g^(S~i,Cj')
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_prime: BigInt,
}

impl Hash for SP2_Shares_Response {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.g_stild.to_bytes().hash(state);
    self.g_stild_prime.to_bytes().hash(state);
  }
}

/// Sub-Protocol 2 - Final result from the websocket
#[derive(Debug, Clone, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SP2_Result_Response {
  pub ballot_valid: bool,
}
