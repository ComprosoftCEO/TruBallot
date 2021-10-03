use actix_web::{http::header::ToStrError, http::StatusCode, HttpResponse, ResponseError};
use diesel::r2d2::PoolError;
use std::{error, fmt};
use uuid_b64::UuidB64 as Uuid;
use validator::ValidationErrors;

use crate::errors::{ErrorResponse, GlobalErrorCode, NamedResourceType, ResourceAction, ResourceType};

#[derive(Debug)]
pub enum ServiceError {
  MissingDatabasePool,
  DatabaseError(diesel::result::Error),
  DatabasePoolError(PoolError),
  StructValidationError(ValidationErrors),
}

impl ServiceError {
  pub fn get_error_response(&self) -> ErrorResponse {
    match self {
      ServiceError::MissingDatabasePool => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "App data is not configured, to configure use App::data()".into(),
        GlobalErrorCode::MissingDatabasePool,
        "Database connection pool 'PgPool' not configured using App::data()".into(),
      ),

      ServiceError::DatabaseError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Query Error".into(),
        GlobalErrorCode::DatabaseError,
        format!("{}", error),
      ),

      ServiceError::DatabasePoolError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Connection Error".into(),
        GlobalErrorCode::DatabaseConnectionError,
        format!("{}", error),
      ),

      ServiceError::StructValidationError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid JSON Object".into(),
        GlobalErrorCode::StructValidationError,
        format!("{}", error),
      ),
    }
  }
}

//
// Various Error Traits
//
impl fmt::Display for ServiceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl ResponseError for ServiceError {
  fn error_response(&self) -> HttpResponse {
    let error = self.get_error_response();
    log::error!("{:?}", error);
    error.error_response()
  }
}

impl error::Error for ServiceError {}

//
// Implicit conversion functions
//
impl From<diesel::result::Error> for ServiceError {
  fn from(error: diesel::result::Error) -> Self {
    ServiceError::DatabaseError(error)
  }
}

impl From<PoolError> for ServiceError {
  fn from(error: PoolError) -> Self {
    ServiceError::DatabasePoolError(error)
  }
}

impl From<ValidationErrors> for ServiceError {
  fn from(error: ValidationErrors) -> Self {
    ServiceError::StructValidationError(error)
  }
}
