use serde_repr::Serialize_repr;

#[derive(Serialize_repr, Debug)]
#[repr(u32)]
pub enum GlobalErrorCode {
  UnknownError,
  MissingDatabasePool,
  DatabaseError,
  DatabaseConnectionError,
  StructValidationError,
}
