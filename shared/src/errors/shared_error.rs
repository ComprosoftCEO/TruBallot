use actix_web::error::{JsonPayloadError, PathError, QueryPayloadError, UrlencodedError};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use diesel::r2d2::PoolError;
use jsonwebtoken::errors::Error as JWTError;
use std::{error, fmt};

use crate::errors::{ErrorResponse, GlobalErrorCode};

/// Internal errors that can occur within the shared library
#[derive(Debug)]
pub enum SharedError {
  DatabaseConnectionError(diesel::ConnectionError),
  DatabasePoolError(PoolError),
  DatabaseError(diesel::result::Error),
  JSONPayloadError(JsonPayloadError),
  FormPayloadError(UrlencodedError),
  URLPathError(PathError),
  QueryStringError(QueryPayloadError),
  JWTError(JWTError),
  JWTExtractorError(AuthenticationError<Bearer>),
}

impl SharedError {
  pub fn get_error_response(&self) -> ErrorResponse {
    match self {
      SharedError::DatabaseConnectionError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Connection Error".into(),
        GlobalErrorCode::DatabaseConnectionError,
        format!("{}", error),
      ),

      SharedError::DatabasePoolError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Connection Error".into(),
        GlobalErrorCode::DatabaseConnectionError,
        format!("{}", error),
      ),

      SharedError::DatabaseError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Query Error".into(),
        GlobalErrorCode::DatabaseQueryError,
        format!("{}", error),
      ),

      SharedError::JSONPayloadError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid JSON Object".into(),
        GlobalErrorCode::JSONPayloadError,
        format!("{}", error),
      ),

      SharedError::FormPayloadError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid Form Data".into(),
        GlobalErrorCode::FormPayloadError,
        format!("{}", error),
      ),

      SharedError::URLPathError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid URL Path".into(),
        GlobalErrorCode::URLPathError,
        format!("{}", error),
      ),

      SharedError::QueryStringError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid Query String".into(),
        GlobalErrorCode::QueryStringError,
        format!("{}", error),
      ),

      SharedError::JWTError(error) => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "Invalid JWT Token".into(),
        GlobalErrorCode::InvalidJWTToken,
        format!("{}", error),
      ),

      SharedError::JWTExtractorError(error) => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "Invalid JWT Token".into(),
        GlobalErrorCode::InvalidJWTToken,
        format!("{}", error),
      ),
    }
  }
}

//
// Various Error Traits
//
impl fmt::Display for SharedError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl ResponseError for SharedError {
  fn error_response(&self) -> HttpResponse {
    let error = self.get_error_response();
    log::error!("{:?}", error);
    error.error_response()
  }
}

impl error::Error for SharedError {}

//
// Implicit conversion functions
//
impl From<diesel::ConnectionError> for SharedError {
  fn from(error: diesel::ConnectionError) -> Self {
    SharedError::DatabaseConnectionError(error)
  }
}

impl From<PoolError> for SharedError {
  fn from(error: PoolError) -> Self {
    SharedError::DatabasePoolError(error)
  }
}

impl From<diesel::result::Error> for SharedError {
  fn from(error: diesel::result::Error) -> Self {
    SharedError::DatabaseError(error)
  }
}

impl From<JsonPayloadError> for SharedError {
  fn from(error: JsonPayloadError) -> Self {
    SharedError::JSONPayloadError(error)
  }
}

impl From<UrlencodedError> for SharedError {
  fn from(error: UrlencodedError) -> Self {
    SharedError::FormPayloadError(error)
  }
}

impl From<PathError> for SharedError {
  fn from(error: PathError) -> Self {
    SharedError::URLPathError(error)
  }
}

impl From<QueryPayloadError> for SharedError {
  fn from(error: QueryPayloadError) -> Self {
    SharedError::QueryStringError(error)
  }
}

impl From<JWTError> for SharedError {
  fn from(error: JWTError) -> Self {
    SharedError::JWTError(error)
  }
}

impl From<AuthenticationError<Bearer>> for SharedError {
  fn from(error: AuthenticationError<Bearer>) -> Self {
    SharedError::JWTExtractorError(error)
  }
}
