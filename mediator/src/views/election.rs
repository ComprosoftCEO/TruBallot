use curv_kzen::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeElectionResult {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub n: BigInt,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelationShares {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_cancelation_shares: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_cancelation_shares: BigInt,
}
