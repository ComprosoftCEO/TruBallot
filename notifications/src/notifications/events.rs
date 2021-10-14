//
// List of all events that can occur in the server
//
use actix::Message;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid_b64::UuidB64 as Uuid;

/// Events not attached to any specific election
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum GlobalEvents {
  ElectionPublished,
}

/// Events specific to an election
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ElectionEvents {
  RegistrationOpened,
  RegistrationCountUpdated,
  RegistrationClosed,
  VotingOpened,
  VoteCountUpdated,
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
}
