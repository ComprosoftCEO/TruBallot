//
// Data structures used to tell the server to broadcast a notification
//
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

/// List of every message that can be sent to the server
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AllMediatorMessages {
  CollectorPublishedOrUpdated(CollectorPublishedOrUpdated),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectorPublishedOrUpdated {
  pub id: Uuid,
  pub name: String,
}
