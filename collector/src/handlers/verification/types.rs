use actix::prelude::*;
use curv_kzen::BigInt;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyBallotData {
  pub user_id: Uuid,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_ballot: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_ballot: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_prime: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_s_prime: BigInt,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyBallotWebsocketData {
  pub user_id: Uuid,

  // Ballots
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_ballot: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_ballot: BigInt,

  // Commitments
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_prime: BigInt,
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_s_s_prime: BigInt,

  // STPM Encryption Key
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub n: BigInt,

  // First step of STPM
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1: BigInt, // E(S_i,C1, e)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1_prime: BigInt, // E(S_i,C1', e)

  // First step of sub-protocol 2
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_1: BigInt, // g^(S~i,C1)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_1_prime: BigInt, // g^(S~i,C1')
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyBallotWebsocketResult {
  // (E(S_i,C1, e)^x2) * (E(r2', e)^(-1)) (mod n^2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1_e_r2_prime: BigInt,

  // (E(S_i,C1', e)^x2) * (E(r2, e)^(-1)) (mod n^2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub e_s_c1_prime_r2: BigInt,

  // Second step of sub-protocol 2
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_2: BigInt, // g^(S~i,C2)
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub g_stild_2_prime: BigInt, // g^(S~i,C2')
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct Product1 {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub p1: BigInt,
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct Product2 {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub p2: BigInt,
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct VerificationResult {
  pub sub_protocol_1: bool,
  pub sub_protocol_2: bool,
}
