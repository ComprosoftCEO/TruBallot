use std::fmt;
use uuid_b64::UuidB64 as Uuid;

use crate::errors::{ResourceType, ServiceError};

/// Resource that can be accessed in the database, with an identifier
#[derive(Debug)]
pub enum NamedResourceType {
  User { id: Uuid },
  Election { id: Uuid },
  Question { id: Uuid, election_id: Uuid },
}

impl NamedResourceType {
  pub fn get_resource_type(&self) -> ResourceType {
    match self {
      NamedResourceType::User { .. } => ResourceType::User,
      NamedResourceType::Election { .. } => ResourceType::Election,
      NamedResourceType::Question { .. } => ResourceType::Question,
    }
  }

  pub fn get_name(&self) -> &'static str {
    self.get_resource_type().get_name()
  }

  pub fn into_error(self) -> ServiceError {
    ServiceError::NoSuchResource(self)
  }

  pub fn user(id: Uuid) -> Self {
    NamedResourceType::User { id }
  }

  pub fn election(id: Uuid) -> Self {
    NamedResourceType::Election { id }
  }

  pub fn question(id: Uuid, election_id: Uuid) -> Self {
    NamedResourceType::Question { id, election_id }
  }
}

impl fmt::Display for NamedResourceType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      NamedResourceType::User { id } => write!(f, "{} (ID: {})", self.get_name(), id),
      NamedResourceType::Election { id } => write!(f, "{} (ID: {})", self.get_name(), id),
      NamedResourceType::Question { id, election_id } => {
        write!(f, "{} (ID: {}, Election ID: {})", self.get_name(), id, election_id)
      }
    }
  }
}
