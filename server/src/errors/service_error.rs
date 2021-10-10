use actix_web::error::{JsonPayloadError, PathError, QueryPayloadError, UrlencodedError};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use bcrypt::BcryptError;
use diesel::r2d2::PoolError;
use jsonwebtoken::errors::Error as JWTError;
use std::{error, fmt};
use uuid_b64::UuidB64 as Uuid;
use validator::ValidationErrors;

use crate::errors::{ErrorResponse, GlobalErrorCode, NamedResourceType, ResourceAction, ResourceType};
use crate::models::ElectionStatus;

/// Enumeration of all possible errors that can occur
#[derive(Debug)]
pub enum ServiceError {
  MissingDatabaseConnectionUrl,
  DatabaseConnectionError(diesel::ConnectionError),
  DatabasePoolError(PoolError),
  DatabaseError(diesel::result::Error),
  SSLConfigurationError(String),
  MissingAppData(String),
  JSONPayloadError(JsonPayloadError),
  FormPayloadError(UrlencodedError),
  URLPathError(PathError),
  QueryStringError(QueryPayloadError),
  StructValidationError(ValidationErrors),
  InvalidEmailPassword,
  JWTError(JWTError),
  JWTExtractorError(AuthenticationError<Bearer>),
  JWTNoSuchUser {
    user_id: Uuid,
  },
  HashPasswordError(BcryptError),
  ZxcvbnError(zxcvbn::ZxcvbnError),
  PasswordComplexityError(zxcvbn::Entropy),
  MissingRecaptchaSecret,
  RecaptchaFailed(recaptcha::Error),
  ForbiddenResourceAction(ResourceType, ResourceAction),
  NoSuchResource(NamedResourceType),
  ElectionNotOwnedByUser {
    current_user_id: Uuid,
    owner_id: Uuid,
    action: ResourceAction,
  },
  ElectionNotDraft {
    election_id: Uuid,
    action: ResourceAction,
  },
  WrongStatusFor {
    election_id: Uuid,
    action: ResourceAction,
    status: ElectionStatus,
  },
  AccessCodeNotFound(String),
  AlreadyRegistered {
    user_id: Uuid,
    election_id: Uuid,
  },
  RegistrationClosed {
    election_id: Uuid,
  },
  NotEnoughRegistered {
    election_id: Uuid,
    num_registered: usize,
  },
}

impl ServiceError {
  pub fn get_error_response(&self) -> ErrorResponse {
    match self {
      ServiceError::MissingDatabaseConnectionUrl => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database Connection Error".into(),
        GlobalErrorCode::DatabaseConnectionError,
        "DATABASE_URL environment variable not set".into(),
      ),

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

      ServiceError::SSLConfigurationError(error) => ErrorResponse::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "SSL Configuration Error".into(),
        GlobalErrorCode::SSLConfigurationError,
        error.clone(),
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

      ServiceError::JWTNoSuchUser { user_id } => ErrorResponse::new(
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

      ServiceError::ElectionNotOwnedByUser {
        current_user_id,
        owner_id,
        action,
      } => ErrorResponse::new(
        StatusCode::CONFLICT,
        format!(
          "Cannot {} election: not owned by current user",
          action.get_name().to_lowercase()
        ),
        GlobalErrorCode::ElectionNotOwnedByUser,
        format!("Current User ID: {}, Owner ID: {}", current_user_id, owner_id),
      ),

      ServiceError::ElectionNotDraft { election_id, action } => ErrorResponse::new(
        StatusCode::CONFLICT,
        format!(
          "Cannot {} election after it has left the draft status",
          action.get_name().to_lowercase()
        ),
        GlobalErrorCode::ElectionNotDraft,
        format!("Election ID: {}", election_id),
      ),

      ServiceError::WrongStatusFor {
        election_id,
        action,
        status,
      } => ErrorResponse::new(
        StatusCode::CONFLICT,
        format!(
          "Cannot {} election in '{}' status",
          action.get_name().to_lowercase(),
          status.get_name().to_lowercase()
        ),
        GlobalErrorCode::WrongElectionStatus,
        format!("Election ID: {}", election_id),
      ),

      ServiceError::AccessCodeNotFound(code) => ErrorResponse::new(
        StatusCode::NOT_FOUND,
        "Invalid access code or code expired".into(),
        GlobalErrorCode::AccessCodeNotFound,
        format!("Access Code: {}", code),
      ),

      ServiceError::AlreadyRegistered { user_id, election_id } => ErrorResponse::new(
        StatusCode::CONFLICT,
        "User is already registered for election".into(),
        GlobalErrorCode::AlreadyRegistered,
        format!("User ID: {}, Electon ID: {}", user_id, election_id),
      ),

      ServiceError::RegistrationClosed { election_id } => ErrorResponse::new(
        StatusCode::CONFLICT,
        "Election registration is closed".into(),
        GlobalErrorCode::RegistrationClosed,
        format!("Electon ID: {}", election_id),
      ),

      ServiceError::NotEnoughRegistered {
        election_id,
        num_registered,
      } => ErrorResponse::new(
        StatusCode::CONFLICT,
        "Need at least two registered users before voting can begin".into(),
        GlobalErrorCode::NotEnoughRegistered,
        format!("Electon ID: {}, Num Registered: {}", election_id, num_registered),
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
