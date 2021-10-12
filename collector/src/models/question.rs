use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::Election;
use crate::schema::questions;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[belongs_to(Election)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Question {
  pub id: Uuid,
  pub election_id: Uuid,
  pub num_candidates: i64,
}

impl Question {
  model_base!();

  belongs_to!(Election);
  has_many!(Registration);

  pub fn new(id: Uuid, election_id: Uuid, num_candidates: i64) -> Self {
    Self {
      id,
      election_id,
      num_candidates,
    }
  }

  /// Search for an election question in the database, and return a ServiceError (not a Diesel error)
  pub fn find_resource(id: &Uuid, election_id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    let question =
      Self::find_optional(id, conn)?.ok_or_else(|| NamedResourceType::question(*id, *election_id).into_error())?;

    // Make sure the election ID matches
    if question.election_id != *election_id {
      return Err(NamedResourceType::question(*id, *election_id).into_error());
    }

    Ok(question)
  }
}
