//
// Data structures that get sent to the client when a notification is broadcasted
//
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid_b64::UuidB64 as Uuid;

use crate::notifications::{ElectionEvents, GlobalEvents};

/// Websocket protocol to specify for the client
///
/// Some browsers (I'm looking at you, Chrome) close the websocket
///  if we don't return one of the websocket protocols from the list
///
/// Since we use the protocol header to send the access token, we
///  need a valid "protocol" to send back to the web browser.
pub const WS_PROTOCOL: &str = "wsevt";

/// ==========================================================
///  Set actor subscriptions - Client sends this to websocket
/// ==========================================================
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
  Replace {
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
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AllClientResponses {
  ElectionCreated(ElectionDetails),
  ElectionPublished(ElectionDetails),
  NameChanged(NameChangedDetails),
  ElectionUpdated(ElectionDetails),
  ElectionDeleted(ElectionDetails),
  RegistrationOpened(ElectionDetails),
  UserRegistered(UserRegisteredDetails),
  UserUnregistered(UserUnregisteredDetails),
  RegistrationClosed(RegistrationClosedDetails),
  VotingOpened(ElectionDetails),
  VoteReceived(VoteReceivedDetails),
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
pub struct NameChangedDetails {
  pub new_name: String,
}

impl From<String> for NameChangedDetails {
  fn from(new_name: String) -> Self {
    Self { new_name }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisteredDetails {
  pub election_id: Uuid,
  pub user_id: Uuid,
  pub user_name: String,
  pub num_registered: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUnregisteredDetails {
  pub election_id: Uuid,
  pub user_id: Uuid,
  pub num_registered: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationClosedDetails {
  pub election_id: Uuid,
  pub is_public: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteReceivedDetails {
  pub election_id: Uuid,
  pub question_id: Uuid,

  pub user_id: Uuid,
  pub user_name: String,
  pub has_voted_status: u32,

  pub forward_ballot: String,
  pub reverse_ballot: String,
  pub g_s: String,
  pub g_s_prime: String,
  pub g_s_s_prime: String,

  pub num_votes: i64,
}
