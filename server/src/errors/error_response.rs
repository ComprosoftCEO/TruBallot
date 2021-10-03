use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Serialize, Serializer};
use std::fmt;

use crate::errors::GlobalErrorCode;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
  #[serde(serialize_with = "serialize_status_code")]
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
