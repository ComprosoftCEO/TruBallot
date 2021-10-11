use serde_repr::{Deserialize_repr, Serialize_repr};

/// Error codes that are exposed on the frontend
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u32)]
pub enum GlobalErrorCode {
  UnknownError = 0,
  DatabaseConnectionError,
  DatabaseQueryError,
  MissingAppData,
  JSONPayloadError,
  FormPayloadError,
  URLPathError,
  QueryStringError,
  StructValidationError,
  InvalidEmailPassword,
  InvalidJWTToken,
  PasswordComplexityError,
  RecaptchaError,
  ForbiddenResourceAction,
  NoSuchResource,
  ElectionNotOwnedByUser,
  ElectionNotDraft,
  WrongElectionStatus,
  AccessCodeNotFound,
  NotRegistered,
  AlreadyRegistered,
  RegistrationClosed,
  NotEnoughRegistered,
  CollectorURLNotSet,
  RegisterElectionError,
  VerificationError,
}
