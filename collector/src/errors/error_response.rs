use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use crate::errors::GlobalErrorCode;

/// JSON response returned to the frontend on an error
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
  #[serde(
    serialize_with = "serialize_status_code",
    deserialize_with = "deserialize_status_code"
  )]
  status_code: StatusCode,
  description: String,
  error_code: GlobalErrorCode,

  // Don't serialize on production system
  #[cfg_attr(not(debug_assertions), serde(skip_serializing))]
  #[serde(skip_serializing_if = "Option::is_none")]
  developer_notes: Option<String>,
}

fn serialize_status_code<S>(code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_u16(code.as_u16())
}

fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
  D: Deserializer<'de>,
{
  let integer: u16 = Deserialize::deserialize(deserializer)?;
  StatusCode::from_u16(integer).map_err(D::Error::custom)
}

impl ErrorResponse {
  pub fn new(
    status_code: StatusCode,
    description: String,
    error_code: GlobalErrorCode,
    developer_notes: String,
  ) -> Self {
    ErrorResponse {
      status_code,
      description,
      error_code,
      developer_notes: Some(developer_notes),
    }
  }
}

impl fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(#{:?}): {}", self.error_code, self.description)?;
    if let Some(ref notes) = self.developer_notes {
      write!(f, "\nDeveloper Notes: {}", notes)?;
    }
    Ok(())
  }
}

impl ResponseError for ErrorResponse {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code).json(&self)
  }
}
