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
}

impl Question {
  model_base!();

  belongs_to!(Election);
}
