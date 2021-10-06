use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum Collector {
  One,
  Two,
}

impl Collector {
  pub fn to_number(&self) -> i32 {
    match self {
      Collector::One => 1,
      Collector::Two => 2,
    }
  }

  pub fn opposite(&self) -> Collector {
    match self {
      Collector::One => Collector::Two,
      Collector::Two => Collector::One,
    }
  }
}

impl fmt::Display for Collector {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Collector::One => write!(f, "①"),
      Collector::Two => write!(f, "②"),
    }
  }
}

impl FromStr for Collector {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input.to_lowercase().as_str() {
      "1" | "one" => Ok(Collector::One),
      "2" | "two" => Ok(Collector::Two),
      _ => Err(format!("Could not parse collector '{}'", input)),
    }
  }
}
