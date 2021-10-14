//
// Data structures that get sent to the client when a notification is broadcasted
//
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid_b64::UuidB64 as Uuid;

use crate::notifications::{ElectionEvents, GlobalEvents};

/// =========================================================
/// Set actor subscriptions - Client sends this to websocket
/// =========================================================
#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SubscriptionActions {
  #[serde(rename_all = "camelCase")]
  Subscribe {
    // Specific global events
    global_events: Option<HashSet<GlobalEvents>>,

    // All events from a given election
    elections: Option<HashSet<Uuid>>,

    // Specific events from a given election
    election_events: Option<HashMap<Uuid, HashSet<ElectionEvents>>>,
  },

  #[serde(rename_all = "camelCase")]
  Unsubscribe {
    // Specific global events
    global_events: Option<HashSet<GlobalEvents>>,

    // All events from a given election
    elections: Option<HashSet<Uuid>>,

    // Specific events from a given election
    election_events: Option<HashMap<Uuid, HashSet<ElectionEvents>>>,
  },

  #[serde(rename_all = "camelCase")]
  UnsubscribeAll,
}

/// =========================================================
///   JSON returned after the subscriptions are updated
/// =========================================================
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WebsocketResponse {
  #[serde(rename_all = "camelCase")]
  Success,

  #[serde(rename_all = "camelCase")]
  Error {
    message: String,

    // Don't serialize on production system
    #[cfg_attr(not(debug_assertions), serde(skip_serializing))]
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_notes: Option<String>,
  },
}

// ==========================================
//      Main Response Data Types
// ==========================================
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AllClientResponses {
  ElectionPublished(ElectionDetails),
  RegistrationOpened(ElectionDetails),
  RegistrationCountUpdated(RegistrationCountUpdated),
  RegistrationClosed(ElectionDetails),
  VotingOpened(ElectionDetails),
  VoteCountUpdated(VoteCountUpdated),
  VotingClosed(ElectionDetails),
  ResultsPublished(ElectionDetails),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionDetails {
  pub election_id: Uuid,
}

impl From<Uuid> for ElectionDetails {
  fn from(election_id: Uuid) -> Self {
    Self { election_id }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationCountUpdated {
  pub election_id: Uuid,
  pub num_registered: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteCountUpdated {
  pub election_id: Uuid,
  pub new_counts: Vec<i64>,
}
