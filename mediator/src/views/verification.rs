use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResult {
  pub sub_protocol_1: bool,
  pub sub_protocol_2: bool,
}
