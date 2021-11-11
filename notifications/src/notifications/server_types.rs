//
// Data structures used to tell the server to broadcast a notification
//
use actix::Message;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;

use crate::notifications::{
  client_types::{self, AllClientResponses},
  ElectionEvent, ElectionEvents, GlobalEvent, GlobalEvents,
};

/// List of every message that can be deserialized from the server
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AllServerMessages {
  ElectionCreated(ElectionCreated),
  ElectionPublished(ElectionPublished),
  NameChanged(NameChanged),
  ElectionUpdated(ElectionUpdated),
  ElectionDeleted(ElectionDeleted),
  RegistrationOpened(RegistrationOpened),
  UserRegistered(UserRegistered),
  UserUnregistered(UserUnregistered),
  RegistrationClosed(RegistrationClosed),
  VotingOpened(VotingOpened),
  VoteReceived(VoteReceived),
  VotingClosed(VotingClosed),
  ResultsPublished(ResultsPublished),
}

///
/// Election Created
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ElectionCreated {
  pub election_id: Uuid,
  pub creator_id: Uuid,
}

impl GlobalEvent for ElectionCreated {
  const EVENT_TYPE: GlobalEvents = GlobalEvents::ElectionCreated;

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::ElectionCreated(self.election_id.into())
  }

  fn protected(&self) -> Option<Uuid> {
    Some(self.creator_id)
  }
}

///
/// Election Published
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ElectionPublished {
  pub election_id: Uuid,
}

impl GlobalEvent for ElectionPublished {
  const EVENT_TYPE: GlobalEvents = GlobalEvents::ElectionPublished;

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::ElectionPublished(self.election_id.into())
  }
}

///
/// Name Changed
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct NameChanged {
  pub user_id: Uuid,
  pub new_name: String,
}

impl GlobalEvent for NameChanged {
  const EVENT_TYPE: GlobalEvents = GlobalEvents::NameChanged;

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::NameChanged(self.new_name.into())
  }

  fn protected(&self) -> Option<Uuid> {
    Some(self.user_id)
  }
}

///
/// Election Updated
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ElectionUpdated {
  pub election_id: Uuid,
}

impl ElectionEvent for ElectionUpdated {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::ElectionUpdated;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::ElectionUpdated(self.election_id.into())
  }
}

///
/// Election Deleted
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ElectionDeleted {
  pub election_id: Uuid,
}

impl ElectionEvent for ElectionDeleted {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::ElectionDeleted;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::ElectionDeleted(self.election_id.into())
  }
}

///
/// Registration Opened
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct RegistrationOpened {
  pub election_id: Uuid,
}

impl ElectionEvent for RegistrationOpened {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::RegistrationOpened;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::RegistrationOpened(self.election_id.into())
  }
}

///
/// User Registered
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct UserRegistered {
  pub election_id: Uuid,
  pub user_id: Uuid,
  pub user_name: String,
  pub num_registered: i64,
}

impl ElectionEvent for UserRegistered {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::UserRegistered;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::UserRegistered(client_types::UserRegisteredDetails {
      election_id: self.election_id,
      user_id: self.user_id,
      user_name: self.user_name,
      num_registered: self.num_registered,
    })
  }
}

///
/// User Unregistered
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct UserUnregistered {
  pub election_id: Uuid,
  pub user_id: Uuid,
  pub num_registered: i64,
}

impl ElectionEvent for UserUnregistered {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::UserUnregistered;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::UserUnregistered(client_types::UserUnregisteredDetails {
      election_id: self.election_id,
      user_id: self.user_id,
      num_registered: self.num_registered,
    })
  }
}

///
/// Registration Closed
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct RegistrationClosed {
  pub election_id: Uuid,
  pub is_public: bool,
}

impl ElectionEvent for RegistrationClosed {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::RegistrationClosed;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::RegistrationClosed(client_types::RegistrationClosedDetails {
      election_id: self.election_id,
      is_public: self.is_public,
    })
  }
}

///
/// Voting Opened
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct VotingOpened {
  pub election_id: Uuid,
}

impl ElectionEvent for VotingOpened {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::VotingOpened;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::VotingOpened(self.election_id.into())
  }
}

///
/// Vote Received
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct VoteReceived {
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

impl ElectionEvent for VoteReceived {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::VoteReceived;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::VoteReceived(client_types::VoteReceivedDetails {
      election_id: self.election_id,
      question_id: self.question_id,

      user_id: self.user_id,
      user_name: self.user_name,
      has_voted_status: self.has_voted_status,

      forward_ballot: self.forward_ballot,
      reverse_ballot: self.reverse_ballot,
      g_s: self.g_s,
      g_s_prime: self.g_s_prime,
      g_s_s_prime: self.g_s_s_prime,

      num_votes: self.num_votes,
    })
  }
}

///
/// Voting Closed
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct VotingClosed {
  pub election_id: Uuid,
}

impl ElectionEvent for VotingClosed {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::VotingClosed;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::VotingClosed(self.election_id.into())
  }
}

///
/// Results Published
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ResultsPublished {
  pub election_id: Uuid,
}

impl ElectionEvent for ResultsPublished {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::ResultsPublished;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::ResultsPublished(self.election_id.into())
  }
}
