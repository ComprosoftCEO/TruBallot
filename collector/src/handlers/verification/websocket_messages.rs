// Define all data structures for the verification protocol (Sub-Protocol 1 and Sub-Protocol 2)
//
// Note: For convention, we denote the client as collector 1 and the websocket actor as collector 2.
//   In reality, these values are always opposite, so they may be flipped.
//   Either way, we still compute the correct verification values.
#![allow(non_camel_case_types)]
use actix::Message;
use curv_kzen::BigInt;
use serde::{Deserialize, Serialize};

/// Initialization parameters to send to the websocket
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct Initialize {
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

  // STPM Encryption Key
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub n: BigInt,
}

/// Sub-Protocol 1 - First Secure Two-Party Multiplication Request
///
/// Computes r1 + r2' = S_i,C1 * S_i,C2'
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SP1_STMP1_Request {
  // E(S_i,C1, e)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1: BigInt,
}

/// Sub-Protocol 1 - First Secure Two-Party Multiplication Response
///
/// Computes r1 + r2' = S_i,C1 * S_i,C2'
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_STMP1_Response {
  // (E(S_i,C1, e)^(S_i,C2')) * (E(r2', e)^(-1)) (mod n^2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1_e_r2_prime: BigInt,
}

/// Sub-Protocol 1 - Second Secure Two-Party Multiplication Request
///
/// Computes r1' + r2 = S_i,C1' * S_i,C2
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SP1_STMP2_Request {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1_prime: BigInt, // E(S_i,C1', e)
}

/// Sub-Protocol 1 - Second Secure Two-Party Multiplication Response
///
/// Computes r1' + r2 = S_i,C1' * S_i,C2
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_STMP2_Response {
  // (E(S_i,C1', e)^(S_i,C2)) * (E(r2, e)^(-1)) (mod n^2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1_prime_e_r2: BigInt,
}

/// Sub-Protocol 1 - Computed product P1 request
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SP1_Product1_Request {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub p1: BigInt,
}

// Sub-Protocol 1 - Computed product P2 response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_Product2_Response {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub p2: BigInt,
}

/// Sub-Protocol 1 - Final result from the websocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP1_Result_Response {
  pub ballot_valid: bool,
}

/// Sub-Protocol 2 - Computed values g^(S~i,C1) and g^(S~i,C1')
#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SP2_C1_Request {
  // g^(S~i,C1)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_1: BigInt,

  // g^(S~i,C1')
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_1_prime: BigInt,
}

/// Sub-Protocol 2 - Computed values g^(S~i,C2) and g^(S~i,C2')
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP2_C2_Response {
  // g^(S~i,C2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_2: BigInt,

  // g^(S~i,C2')
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_2_prime: BigInt,
}

/// Sub-Protocol 2 - Final result from the websocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SP2_Result_Response {
  pub ballot_valid: bool,
}
