use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

pub static DEFAULT_PERMISSIONS: &[Permission] = &[
  Permission::CanLogin,
  Permission::CreateElection,
  Permission::Register,
  Permission::Vote,
];

/// List of every permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Permission {
  CanLogin,
  CreateElection,
  Register,
  Vote,
}

impl fmt::Display for Permission {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
