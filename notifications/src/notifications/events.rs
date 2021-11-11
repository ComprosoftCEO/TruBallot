//
// List of all events that can occur in the server
//
use actix::Message;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;

/// Events not attached to any specific election
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GlobalEvents {
  ElectionCreated,
  ElectionPublished,
  NameChanged,
}

/// Events specific to an election
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElectionEvents {
  ElectionUpdated,
  ElectionDeleted,
  RegistrationOpened,
  UserRegistered,
  UserUnregistered,
  RegistrationClosed,
  VotingOpened,
  VoteReceived,
  VotingClosed,
  ResultsPublished,
}

/// Wraps a global event
#[derive(Message)]
#[rtype(result = "()")]
pub struct GlobalEventWrapper<T: GlobalEvent>(pub T);

/// Wraps an election event
#[derive(Message)]
#[rtype(result = "()")]
pub struct ElectionEventWrapper<T: ElectionEvent>(pub T);

///
/// All global event data structures must implement this trait
///
pub trait GlobalEvent: Sized {
  const EVENT_TYPE: GlobalEvents;

  type Output: Serialize;
  fn into_output(self) -> Self::Output;

  /// Wrap inside a structure that can be passed generically to the subscription actor
  fn wrap(self) -> GlobalEventWrapper<Self> {
    GlobalEventWrapper(self)
  }

  /// Override this method to have a protected event
  fn protected(&self) -> Option<Uuid> {
    None
  }
}

///
/// All election event data structures must implement this trait
///
pub trait ElectionEvent: Sized {
  const EVENT_TYPE: ElectionEvents;

  fn get_election_id(&self) -> Uuid;

  type Output: Serialize;
  fn into_output(self) -> Self::Output;

  /// Wrap inside a structure that can be passed generically to the subscription actor
  fn wrap(self) -> ElectionEventWrapper<Self> {
    ElectionEventWrapper(self)
  }

  /// Override this method to have an event protected by a user ID
  fn protected(&self) -> Option<Uuid> {
    None
  }
}
