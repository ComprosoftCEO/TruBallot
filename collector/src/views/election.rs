use curv_kzen::BigInt;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionResponse {
  // Vector might be empty when returning from the second collector
  #[serde(with = "kzen_paillier::serialize::vecbigint")]
  pub encryption_result: Vec<BigInt>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionParameters {
  #[serde(
    skip_serializing_if = "Option::is_none",
    with = "crate::utils::serialize_option_bigint"
  )]
  pub encrypted_location: Option<BigInt>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionParameters {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_verification_shares: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_verification_shares: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_ballot_shares: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_ballot_shares: BigInt,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelationShares {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub forward_cancelation_shares: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  pub reverse_cancelation_shares: BigInt,
}
