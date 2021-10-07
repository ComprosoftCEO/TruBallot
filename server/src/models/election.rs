use rand::{thread_rng, Rng};
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::{ElectionStatus, User};
use crate::schema::elections;
use crate::utils::new_safe_uuid_v4;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[belongs_to(User, foreign_key = "created_by")]
#[serde(rename_all = "camelCase")]
pub struct Election {
  pub id: Uuid,
  pub name: String,
  pub created_by: Uuid,
  pub status: ElectionStatus,
  pub encryption_key: Vec<u8>,
}

impl Election {
  model_base!();

  belongs_to!(User);
  has_many!(Question);
  has_many!(Registration);
  has_many!(Commitment);

  pub fn new(name: impl Into<String>, created_by: Uuid) -> Self {
    // Generate a random AES encryption key
    let encryption_key = thread_rng().gen::<[u8; 32]>().to_vec();

    Self {
      id: new_safe_uuid_v4(),
      name: name.into(),
      created_by,
      status: ElectionStatus::Registration,
      encryption_key,
    }
  }
}
