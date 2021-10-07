use serde_repr::Serialize_repr;

/// Error codes that are exposed on the frontend
#[derive(Serialize_repr, Debug)]
#[repr(u32)]
pub enum GlobalErrorCode {
  UnknownError,
  DatabaseConnectionError,
  DatabaseQueryError,
  SSLConfigurationError,
  MissingAppData,
  JSONPayloadError,
  StructValidationError,
  InvalidEmailPassword,
  InvalidJWTToken,
  PasswordComplexityError,
  RecaptchaError,
}
