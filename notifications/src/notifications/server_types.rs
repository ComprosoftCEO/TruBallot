//
// Data structures used to tell the server to broadcast a notification
//
use actix::Message;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;

use crate::notifications::{client_types, ElectionEvent, ElectionEvents, GlobalEvent, GlobalEvents};

/// List of every message that can be deserialized from the server
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AllServerMessages {
  ElectionPublished(ElectionPublished),
  RegistrationOpened(RegistrationOpened),
}

#[derive(Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ElectionPublished {
  pub id: Uuid,
}

impl GlobalEvent for ElectionPublished {
  const EVENT_TYPE: GlobalEvents = GlobalEvents::ElectionPublished;

  type Output = client_types::ElectionPublished;
  fn into_output(self) -> Self::Output {
    Self::Output { id: self.id }
  }
}

#[derive(Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct RegistrationOpened {
  pub id: Uuid,
}

impl ElectionEvent for RegistrationOpened {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::RegistrationOpened;

  fn get_election_id(&self) -> Uuid {
    self.id
  }

  type Output = client_types::RegistrationOpened;
  fn into_output(self) -> Self::Output {
    Self::Output { id: self.id }
  }
}
