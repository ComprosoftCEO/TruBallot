use std::fmt;

/// Resource that can be accessed in the database, without any identifiers
#[derive(Debug)]
pub enum ResourceType {
  Collector,
  User,
  Election,
  Question,
  Registration,
}

impl ResourceType {
  pub fn get_name(&self) -> &'static str {
    match self {
      ResourceType::Collector => "Collector",
      ResourceType::User => "User",
      ResourceType::Election => "Election",
      ResourceType::Question => "Question",
      ResourceType::Registration => "Registration",
    }
  }
}

impl fmt::Display for ResourceType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.get_name())
  }
}
