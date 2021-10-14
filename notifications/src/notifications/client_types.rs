//
// Data structures that get sent to the client when a notification is broadcasted
//
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid_b64::UuidB64 as Uuid;

use crate::notifications::{ElectionEvents, GlobalEvents};

/// Set actor subscriptions
#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SubscriptionActions {
  Subscribe {
    // Specific global events
    global_events: HashSet<GlobalEvents>,

    // All events from a given election
    elections: HashSet<Uuid>,

    // Specific events from a given election
    election_events: HashMap<Uuid, HashSet<ElectionEvents>>,
  },
  Unsubscribe {
    // Specific global events
    global_events: HashSet<GlobalEvents>,

    // All events from a given election
    elections: HashSet<Uuid>,

    // Specific events from a given election
    election_events: HashMap<Uuid, HashSet<ElectionEvents>>,
  },
  UnsubscribeAll,
}

/// JSON returned after the subscriptions are updated
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WebsocketResponse {
  Success,
  Error {
    message: String,

    // Don't serialize on production system
    #[cfg_attr(not(debug_assertions), serde(skip_serializing))]
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_notes: Option<String>,
  },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionPublished {
  pub id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationOpened {
  pub id: Uuid,
}
