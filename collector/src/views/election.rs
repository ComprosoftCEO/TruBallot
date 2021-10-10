use aes::BLOCK_SIZE;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionResponse {
  pub encrypted_positions: Vec<[u8; BLOCK_SIZE]>,
}
