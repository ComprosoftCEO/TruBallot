use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::Collector;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicCollectorList {
  pub id: Uuid,
  pub name: String,
}

pub type PublicCollector = PublicCollectorList;

impl PublicCollectorList {
  pub fn from_collector(c: Collector) -> Self {
    Self { id: c.id, name: c.name }
  }
}
