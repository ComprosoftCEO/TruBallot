use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

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
}
