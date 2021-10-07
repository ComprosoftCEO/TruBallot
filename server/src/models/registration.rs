use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::ManyToManyConstructor;
use crate::models::{Election, User};
use crate::schema::registrations;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[primary_key(user_id, election_id)]
#[belongs_to(User)]
#[belongs_to(Election)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Registration {
  pub user_id: Uuid,
  pub election_id: Uuid,
  pub encrypted_location: Vec<u8>,
  pub has_voted: bool,
}

impl Registration {
  model_base!();

  belongs_to!(User);
  belongs_to!(Election);

  // Sadly, our ORM can't represent this relationship
  // has_many!(Commitment);

  pub fn new(user_id: Uuid, election_id: Uuid) -> Self {
    Self {
      user_id,
      election_id,
      encrypted_location: Vec::new(),
      has_voted: false,
    }
  }
}

impl ManyToManyConstructor<User, Election> for Registration {
  fn new(user_id: &Uuid, election_id: &Uuid) -> Self {
    Self::new(*user_id, *election_id)
  }
}

impl ManyToManyConstructor<Election, User> for Registration {
  fn new(election_id: &Uuid, user_id: &Uuid) -> Self {
    Self::new(*user_id, *election_id)
  }
}
