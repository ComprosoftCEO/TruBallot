use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::ManyToManyConstructor;
use crate::models::{Election, User};
use crate::schema::registrations;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, Associations)]
#[primary_key(user_id, election_id)]
#[belongs_to(User)]
#[belongs_to(Election)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
  pub user_id: Uuid,
  pub election_id: Uuid,
}

impl Registration {
  model_base!(no update);

  belongs_to!(User);
  belongs_to!(Election);

  // Sadly, our ORM can't represent this relationship
  // has_many!(Commitment);

  pub fn new(user_id: Uuid, election_id: Uuid) -> Self {
    Self { user_id, election_id }
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
