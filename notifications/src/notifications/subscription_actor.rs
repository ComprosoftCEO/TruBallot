use actix::prelude::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid_b64::UuidB64 as Uuid;

use super::internal_types::{Notify, Replace, Subscribe, Unsubscribe, UnsubscribeAll};
use crate::notifications::{
  ElectionEvent, ElectionEventWrapper, ElectionEvents, GlobalEvent, GlobalEventWrapper, GlobalEvents,
};

/// Actor that stores all of the active subscriptions
pub struct SubscriptionActor {
  global_subscriptions: HashMap<GlobalEvents, HashSet<Recipient<Notify>>>,
  election_subscriptions: HashMap<Uuid, HashSet<Recipient<Notify>>>,
  election_event_subscriptions: HashMap<Uuid, HashMap<ElectionEvents, HashSet<Recipient<Notify>>>>,
}

impl SubscriptionActor {
  pub fn new() -> Self {
    Self {
      global_subscriptions: HashMap::new(),
      election_subscriptions: HashMap::new(),
      election_event_subscriptions: HashMap::new(),
    }
  }

  /// Internal method to send JSON data to a list of subscribers
  fn send_all<'a, T, I>(data: &T, subscribers: I)
  where
    T: ?Sized + Serialize,
    I: IntoIterator<Item = &'a Recipient<Notify>>,
  {
    // Convert the data structure to a JSON string
    let json = match serde_json::to_string(data) {
      Ok(json) => json,
      Err(e) => return log::error!("Failed to serialize JSON data: {}", e),
    };

    // Notify all of the websockets
    let json = Arc::new(json);
    for subscriber in subscribers.into_iter() {
      subscriber.do_send(Notify(json.clone())).ok();
    }
  }
}

impl Actor for SubscriptionActor {
  type Context = Context<Self>;
}

//
// Handle Subscriptions
//
impl Handler<Subscribe> for SubscriptionActor {
  type Result = ();

  fn handle(&mut self, subscribe: Subscribe, _ctx: &mut Self::Context) -> Self::Result {
    let Subscribe {
      me,
      global_events,
      elections,
      election_events,
    } = subscribe;

    // Subscribe to global events
    if let Some(global_events) = global_events {
      for event in global_events {
        let set = self.global_subscriptions.entry(event).or_insert_with(|| HashSet::new());

        set.insert(me.clone());
      }
    }

    // Subscribe to all election events
    if let Some(elections) = elections {
      for election_id in elections {
        let set = self
          .election_subscriptions
          .entry(election_id)
          .or_insert_with(|| HashSet::new());

        set.insert(me.clone());
      }
    }

    // Subscribe to specific election events
    if let Some(election_events) = election_events {
      for (election_id, election_events) in election_events {
        let map = self
          .election_event_subscriptions
          .entry(election_id)
          .or_insert_with(|| HashMap::new());

        for event in election_events {
          let set = map.entry(event).or_insert_with(|| HashSet::new());
          set.insert(me.clone());
        }
      }
    }
  }
}

impl Handler<Unsubscribe> for SubscriptionActor {
  type Result = ();

  fn handle(&mut self, unsubscribe: Unsubscribe, _ctx: &mut Self::Context) -> Self::Result {
    let Unsubscribe {
      me,
      global_events,
      elections,
      election_events,
    } = unsubscribe;

    // Unsubscribe from global events
    if let Some(global_events) = global_events {
      for event in global_events {
        if let Some(set) = self.global_subscriptions.get_mut(&event) {
          set.remove(&me);
        }
      }
    }

    // Unsubscribe from all election events
    if let Some(elections) = elections {
      for election_id in elections {
        if let Some(set) = self.election_subscriptions.get_mut(&election_id) {
          set.remove(&me);
        }
      }
    }

    // Unsubscribe from specific election events
    if let Some(election_events) = election_events {
      for (election_id, election_events) in election_events {
        if let Some(map) = self.election_event_subscriptions.get_mut(&election_id) {
          for event in election_events {
            if let Some(set) = map.get_mut(&event) {
              set.remove(&me);
            }
          }
        }
      }
    }
  }
}

impl Handler<Replace> for SubscriptionActor {
  type Result = ();

  fn handle(&mut self, replace: Replace, _ctx: &mut Self::Context) -> Self::Result {
    let Replace {
      me,
      global_events,
      elections,
      election_events,
    } = replace;

    // Replace global events
    for (_, events) in self.global_subscriptions.iter_mut() {
      events.remove(&me);
    }

    if let Some(global_events) = global_events {
      for event in global_events {
        let set = self.global_subscriptions.entry(event).or_insert_with(|| HashSet::new());

        set.insert(me.clone());
      }
    }

    // Replace global election events
    for (_, events) in self.election_subscriptions.iter_mut() {
      events.remove(&me);
    }

    if let Some(elections) = elections {
      for election_id in elections {
        let set = self
          .election_subscriptions
          .entry(election_id)
          .or_insert_with(|| HashSet::new());

        set.insert(me.clone());
      }
    }

    // Replace specific election events
    for (_, election_events) in self.election_event_subscriptions.iter_mut() {
      for (_, events) in election_events {
        events.remove(&me);
      }
    }

    if let Some(election_events) = election_events {
      for (election_id, election_events) in election_events {
        let map = self
          .election_event_subscriptions
          .entry(election_id)
          .or_insert_with(|| HashMap::new());

        for event in election_events {
          let set = map.entry(event).or_insert_with(|| HashSet::new());
          set.insert(me.clone());
        }
      }
    }
  }
}

impl Handler<UnsubscribeAll> for SubscriptionActor {
  type Result = ();

  fn handle(&mut self, unsubscribe_all: UnsubscribeAll, _ctx: &mut Self::Context) -> Self::Result {
    let UnsubscribeAll { me } = unsubscribe_all;

    // Global events
    for (_, events) in self.global_subscriptions.iter_mut() {
      events.remove(&me);
    }

    // Election events
    for (_, events) in self.election_subscriptions.iter_mut() {
      events.remove(&me);
    }

    for (_, election_events) in self.election_event_subscriptions.iter_mut() {
      for (_, events) in election_events {
        events.remove(&me);
      }
    }
  }
}

//
// Handle server event types
//
impl<T: GlobalEvent> Handler<GlobalEventWrapper<T>> for SubscriptionActor {
  type Result = ();

  fn handle(&mut self, event: GlobalEventWrapper<T>, _ctx: &mut Self::Context) -> Self::Result {
    if let Some(clients) = self.global_subscriptions.get(&T::EVENT_TYPE) {
      Self::send_all(&event.0.into_output(), clients)
    }
  }
}

impl<T: ElectionEvent> Handler<ElectionEventWrapper<T>> for SubscriptionActor {
  type Result = ();

  fn handle(&mut self, event: ElectionEventWrapper<T>, _ctx: &mut Self::Context) -> Self::Result {
    let election_id = event.0.get_election_id();
    let output = event.0.into_output();

    // Keep track of the unique clients
    let mut all_clients = HashSet::new();

    // Clients subscribed to all election events
    if let Some(clients) = self.election_subscriptions.get(&election_id) {
      all_clients.extend(clients);
    }

    // Clients subscribed to the specific event
    if let Some(map) = self.election_event_subscriptions.get(&election_id) {
      if let Some(clients) = map.get(&T::EVENT_TYPE) {
        all_clients.extend(clients);
      }
    }

    // Send the JSON data
    Self::send_all(&output, all_clients)
  }
}
