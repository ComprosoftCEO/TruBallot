use aes::BLOCK_SIZE;
use curv_kzen::BigInt;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionResponse {
  pub encrypted_locations: Vec<[u8; BLOCK_SIZE]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionParameters {
  pub encryption_key: String,
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
