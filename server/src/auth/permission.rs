use serde::{Deserialize, Serialize};
use std::fmt;

/// List of every permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
  CanLogin,
  CreateElection,
  Vote,
}

impl fmt::Display for Permission {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
