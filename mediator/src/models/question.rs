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
}

impl Question {
  model_base!();

  belongs_to!(Election);

  pub fn new(id: Uuid, election_id: Uuid) -> Self {
    Self { id, election_id }
  }

  pub fn find_resource(id: &Uuid, election_id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    Question::find_optional(id, conn)?
      .and_then(|q| if q.election_id != *election_id { None } else { Some(q) })
      .ok_or_else(|| NamedResourceType::question(*id, *election_id).into_error())
  }
}
