use curv_kzen::BigInt;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

/// Data to send to the collector to verify the ballot
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerifyBallotData {
  pub user_id: Uuid,

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
}
