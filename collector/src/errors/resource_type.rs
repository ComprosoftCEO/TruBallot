use std::fmt;

/// Resource that can be accessed in the database, without any identifiers
#[derive(Debug)]
pub enum ResourceType {
  User,
  Election,
  Question,
  EncryptedLocation,
}

impl ResourceType {
  pub fn get_name(&self) -> &'static str {
    match self {
      ResourceType::User => "User",
      ResourceType::Election => "Election",
      ResourceType::Question => "Question",
      ResourceType::EncryptedLocation => "Encrypted location",
    }
  }
}

impl fmt::Display for ResourceType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.get_name())
  }
}
