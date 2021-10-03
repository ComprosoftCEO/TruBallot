use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use bcrypt::BcryptError;
use diesel::r2d2::PoolError;
use jsonwebtoken::errors::Error as JWTError;
use std::{error, fmt};
use uuid_b64::UuidB64 as Uuid;
use validator::ValidationErrors;

use crate::errors::{ErrorResponse, GlobalErrorCode};

/// Enumeration of all possible errors that can occur
#[derive(Debug)]
pub enum ServiceError {
  MissingDatabasePool,
  DatabaseError(diesel::result::Error),
  DatabasePoolError(PoolError),
  StructValidationError(ValidationErrors),
  InvalidEmailPassword,
  JWTError(JWTError),
  JWTExtractorError(AuthenticationError<Bearer>),
  JWTNoSuchUser(Uuid),
  HashPasswordError(BcryptError),
  ZxcvbnError(zxcvbn::ZxcvbnError),
  PasswordComplexityError(zxcvbn::Entropy),
  MissingRecaptchaSecret,
  RecaptchaFailed(recaptcha::Error),
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

      ServiceError::JWTNoSuchUser(user_id) => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "Invalid JWT Token".into(),
        GlobalErrorCode::InvalidJWTToken,
        format!("No Such User: {}", user_id),
      ),

      ServiceError::HashPasswordError(error) => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "Invalid email or password".into(),
        GlobalErrorCode::InvalidEmailPassword,
        format!("Password Hashing Failed: {}", error),
      ),

      ServiceError::ZxcvbnError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Error verifying password complexity".into(),
        GlobalErrorCode::PasswordComplexityError,
        format!("{}", error),
      ),

      ServiceError::PasswordComplexityError(entropy) => ErrorResponse::new(
        StatusCode::BAD_REQUEST,
        "Password failed complexity test — use a different password".into(),
        GlobalErrorCode::PasswordComplexityError,
        format!("{:?}", entropy),
      ),

      ServiceError::MissingRecaptchaSecret => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Server Misconfiguration: Missing reCAPTCHA Secret".into(),
        GlobalErrorCode::RecaptchaError,
        "".into(),
      ),

      ServiceError::RecaptchaFailed(error) => ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        "reCAPTCHA Failed to Validate".into(),
        GlobalErrorCode::RecaptchaError,
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

impl From<BcryptError> for ServiceError {
  fn from(error: BcryptError) -> Self {
    ServiceError::HashPasswordError(error)
  }
}

impl From<zxcvbn::ZxcvbnError> for ServiceError {
  fn from(error: zxcvbn::ZxcvbnError) -> Self {
    ServiceError::ZxcvbnError(error)
  }
}

impl From<zxcvbn::Entropy> for ServiceError {
  fn from(error: zxcvbn::Entropy) -> Self {
    ServiceError::PasswordComplexityError(error)
  }
}

impl From<recaptcha::Error> for ServiceError {
  fn from(error: recaptcha::Error) -> Self {
    ServiceError::RecaptchaFailed(error)
  }
}
