use std::fmt;

/// Actions that can be taken in the database
#[derive(Debug)]
pub enum ResourceAction {
  Create,
  ReadPrivate,
  Update,
  Delete,
  OpenRegistration,
  Register,
  InitVoting,
}

impl ResourceAction {
  pub fn get_name(&self) -> &'static str {
    match self {
      ResourceAction::Create => "Create",
      ResourceAction::ReadPrivate => "Read",
      ResourceAction::Update => "Update",
      ResourceAction::Delete => "Delete",
      ResourceAction::OpenRegistration => "Open Registration for",
      ResourceAction::Register => "Register for",
      ResourceAction::InitVoting => "Initialize voting for",
    }
  }
}

impl fmt::Display for ResourceAction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.get_name())
  }
}
