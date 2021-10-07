use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewElectionResult {
  pub id: Uuid,
}
