use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::Election;
use crate::schema::registrations;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, Associations)]
#[primary_key(user_id, election_id)]
#[belongs_to(Election)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
  pub user_id: Uuid,
  pub election_id: Uuid,
}

impl Registration {
  model_base!(no update);

  belongs_to!(Election);

  pub fn new(user_id: Uuid, election_id: Uuid) -> Self {
    Self { user_id, election_id }
  }
}
