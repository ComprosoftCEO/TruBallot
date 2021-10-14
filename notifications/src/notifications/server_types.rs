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
  ElectionPublished(ElectionPublished),
  RegistrationOpened(RegistrationOpened),
  RegistrationCountUpdated(RegistrationCountUpdated),
  RegistrationClosed(RegistrationClosed),
  VotingOpened(VotingOpened),
  VoteCountUpdated(VoteCountUpdated),
  VotingClosed(VotingClosed),
  ResultsPublished(ResultsPublished),
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
/// Registration Count Updated
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct RegistrationCountUpdated {
  pub election_id: Uuid,
  pub num_registered: i64,
}

impl ElectionEvent for RegistrationCountUpdated {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::RegistrationCountUpdated;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::RegistrationCountUpdated(client_types::RegistrationCountUpdated {
      election_id: self.election_id,
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
}

impl ElectionEvent for RegistrationClosed {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::RegistrationClosed;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::RegistrationClosed(self.election_id.into())
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
/// Vote Count Updated
///
#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct VoteCountUpdated {
  pub election_id: Uuid,
  pub new_counts: Vec<i64>,
}

impl ElectionEvent for VoteCountUpdated {
  const EVENT_TYPE: ElectionEvents = ElectionEvents::VoteCountUpdated;

  fn get_election_id(&self) -> Uuid {
    self.election_id
  }

  type Output = AllClientResponses;
  fn into_output(self) -> Self::Output {
    AllClientResponses::VoteCountUpdated(client_types::VoteCountUpdated {
      election_id: self.election_id,
      new_counts: self.new_counts,
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
