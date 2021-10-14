//
// Internal messages shared between the two actors
//
use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid_b64::UuidB64 as Uuid;

use crate::notifications::{ElectionEvents, GlobalEvents};

/// Send a text JSON notification to the websocket actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct Notify(pub Arc<String>);

/// Add the actor to specific types of messages
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Subscribe {
  pub me: Recipient<Notify>,

  // Specific global events
  pub global_events: HashSet<GlobalEvents>,

  // All events from a given election
  pub elections: HashSet<Uuid>,

  // Specific events from a given election
  pub election_events: HashMap<Uuid, HashSet<ElectionEvents>>,
}

/// Remove the actor from specific types of messages
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Unsubscribe {
  pub me: Recipient<Notify>,

  // Specific global events
  pub global_events: HashSet<GlobalEvents>,

  // All events from a given election
  pub elections: HashSet<Uuid>,

  // Specific events from a given election
  pub election_events: HashMap<Uuid, HashSet<ElectionEvents>>,
}

/// Unsubscribe the actor from all messages
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct UnsubscribeAll {
  pub me: Recipient<Notify>,
}
