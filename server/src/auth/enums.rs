use serde::{Deserialize, Serialize};
use std::fmt;

/// List of every permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
  CanLogin,
  CreateElection,
  Vote,
}

/// List of all audience types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Audience {
  All,
  Internal,
}

impl fmt::Display for Permission {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl fmt::Display for Audience {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
