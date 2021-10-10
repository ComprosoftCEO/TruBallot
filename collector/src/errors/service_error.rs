use actix_web::error::{JsonPayloadError, PathError, QueryPayloadError, UrlencodedError};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use diesel::r2d2::PoolError;
use jsonwebtoken::errors::Error as JWTError;
use std::{error, fmt};
use validator::ValidationErrors;

use crate::errors::{ErrorResponse, GlobalErrorCode, NamedResourceType, ResourceAction, ResourceType};
use crate::Collector;

/// Enumeration of all possible errors that can occur
#[derive(Debug)]
pub enum ServiceError {
  DatabaseConnectionError(diesel::ConnectionError),
  DatabasePoolError(PoolError),
  DatabaseError(diesel::result::Error),
  MissingAppData(String),
  JSONPayloadError(JsonPayloadError),
  FormPayloadError(UrlencodedError),
  URLPathError(PathError),
  QueryStringError(QueryPayloadError),
  StructValidationError(ValidationErrors),
  InvalidEmailPassword,
  JWTError(JWTError),
  JWTExtractorError(AuthenticationError<Bearer>),
  ForbiddenResourceAction(ResourceType, ResourceAction),
  NoSuchResource(NamedResourceType),
  CollectorURLNotSet(Collector),
}

impl ServiceError {
  pub fn get_error_response(&self) -> ErrorResponse {
    match self {
      ServiceError::DatabaseConnectionError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Connection Error".into(),
        GlobalErrorCode::DatabaseConnectionError,
        format!("{}", error),
      ),

      ServiceError::DatabasePoolError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Connection Error".into(),
        GlobalErrorCode::DatabaseConnectionError,
        format!("{}", error),
      ),

      ServiceError::DatabaseError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Query Error".into(),
        GlobalErrorCode::DatabaseQueryError,
        format!("{}", error),
      ),

      ServiceError::MissingAppData(data) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Server Misconfiguration".into(),
        GlobalErrorCode::MissingAppData,
        format!("'{}' not configured using App::data()", data),
      ),

      ServiceError::JSONPayloadError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid JSON Object".into(),
        GlobalErrorCode::JSONPayloadError,
        format!("{}", error),
      ),

      ServiceError::FormPayloadError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid Form Data".into(),
        GlobalErrorCode::FormPayloadError,
        format!("{}", error),
      ),

      ServiceError::URLPathError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid URL Path".into(),
        GlobalErrorCode::URLPathError,
        format!("{}", error),
      ),

      ServiceError::QueryStringError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid Query String".into(),
        GlobalErrorCode::QueryStringError,
        format!("{}", error),
      ),

      ServiceError::StructValidationError(error) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Invalid JSON Object".into(),
        GlobalErrorCode::StructValidationError,
        format!("{}", error),
      ),

      ServiceError::InvalidEmailPassword => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "Invalid email or password".into(),
        GlobalErrorCode::InvalidEmailPassword,
        "".into(),
      ),

      ServiceError::JWTError(error) => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "Invalid JWT Token".into(),
        GlobalErrorCode::InvalidJWTToken,
        format!("{}", error),
      ),

      ServiceError::JWTExtractorError(error) => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "Invalid JWT Token".into(),
        GlobalErrorCode::InvalidJWTToken,
        format!("{}", error),
      ),

      ServiceError::ForbiddenResourceAction(resource, action) => ErrorResponse::new(
        StatusCode::FORBIDDEN,
        format!("Forbidden Action: {} {}", action, resource),
        GlobalErrorCode::ForbiddenResourceAction,
        format!("{} {}", action, resource),
      ),

      ServiceError::NoSuchResource(resource) => ErrorResponse::new(
        StatusCode::NOT_FOUND,
        format!("No Such {}", resource.get_resource_type()),
        GlobalErrorCode::NoSuchResource,
        format!("{}", resource),
      ),

      ServiceError::CollectorURLNotSet(collector) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Server Misconfiguration".into(),
        GlobalErrorCode::CollectorURLNotSet,
        format!("{} environment variable not set", collector.env_prefix("URL")),
      ),
    }
  }
}

//
// Various Error Traits
//
impl fmt::Display for ServiceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.get_error_response())
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
impl From<diesel::ConnectionError> for ServiceError {
  fn from(error: diesel::ConnectionError) -> Self {
    ServiceError::DatabaseConnectionError(error)
  }
}

impl From<PoolError> for ServiceError {
  fn from(error: PoolError) -> Self {
    ServiceError::DatabasePoolError(error)
  }
}

impl From<diesel::result::Error> for ServiceError {
  fn from(error: diesel::result::Error) -> Self {
    ServiceError::DatabaseError(error)
  }
}

impl From<JsonPayloadError> for ServiceError {
  fn from(error: JsonPayloadError) -> Self {
    ServiceError::JSONPayloadError(error)
  }
}

impl From<UrlencodedError> for ServiceError {
  fn from(error: UrlencodedError) -> Self {
    ServiceError::FormPayloadError(error)
  }
}

impl From<PathError> for ServiceError {
  fn from(error: PathError) -> Self {
    ServiceError::URLPathError(error)
  }
}

impl From<QueryPayloadError> for ServiceError {
  fn from(error: QueryPayloadError) -> Self {
    ServiceError::QueryStringError(error)
  }
}

impl From<ValidationErrors> for ServiceError {
  fn from(error: ValidationErrors) -> Self {
    ServiceError::StructValidationError(error)
  }
}

impl From<JWTError> for ServiceError {
  fn from(error: JWTError) -> Self {
    ServiceError::JWTError(error)
  }
}

impl From<AuthenticationError<Bearer>> for ServiceError {
  fn from(error: AuthenticationError<Bearer>) -> Self {
    ServiceError::JWTExtractorError(error)
  }
}
