use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::str::FromStr;

use crate::config;
use crate::errors::ServiceError;

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

  /// Get the prefix to use when loading collector-specific environment variables
  pub fn env_prefix(&self, env: &str) -> String {
    format!("C{}_{}", self.to_number(), env)
  }

  /// Build the URL for a given collector, relative from the root API endpoint
  ///
  /// _Note:_ The root endpoint is `/api/v1/collector/{1 or 2}`
  pub fn api_url(&self, url: &str) -> Result<String, ServiceError> {
    let url_base = match self {
      Collector::One => config::get_c1_url(),
      Collector::Two => config::get_c2_url(),
    }
    .ok_or_else(|| ServiceError::CollectorURLNotSet(*self))?;

    Ok(format!("{}/api/v1/collector/{}{}", url_base, self.to_number(), url))
  }

  /// Test if the collector us using TLS or not
  pub fn is_secure(&self) -> bool {
    std::env::var(self.env_prefix("USE_HTTPS"))
      .map(|https| https.parse().unwrap_or(false))
      .unwrap_or(false)
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
