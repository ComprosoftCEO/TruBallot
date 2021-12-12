use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::schema::collectors;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Collector {
  pub id: Uuid,
  pub name: String,
  pub private_base_url: String,
  pub is_secure: bool,
}

impl Collector {
  model_base!();

  has_many!(Election through ElectionCollector);
}
