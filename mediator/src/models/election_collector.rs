use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::ManyToManyConstructor;
use crate::models::{Collector, Election};
use crate::schema::election_collectors;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, Associations)]
#[primary_key(election_id, collector_id)]
#[belongs_to(Election)]
#[belongs_to(Collector)]
#[serde(rename_all = "camelCase")]
pub struct ElectionCollector {
  pub election_id: Uuid,
  pub collector_id: Uuid,
}

impl ElectionCollector {
  model_base!(no update);

  belongs_to!(Election);
  belongs_to!(Collector);

  pub fn new(election_id: Uuid, collector_id: Uuid) -> Self {
    Self {
      election_id,
      collector_id,
    }
  }
}

impl ManyToManyConstructor<Election, Collector> for ElectionCollector {
  fn new(election_id: &Uuid, collector_id: &Uuid) -> Self {
    Self::new(*election_id, *collector_id)
  }
}

impl ManyToManyConstructor<Collector, Election> for ElectionCollector {
  fn new(collector_id: &Uuid, election_id: &Uuid) -> Self {
    Self::new(*election_id, *collector_id)
  }
}
