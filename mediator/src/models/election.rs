use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::schema::elections;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Election {
  pub id: Uuid,
  pub is_public: bool,
}

impl Election {
  model_base!();

  has_many!(Question);
  has_many!(Registration);
  has_many!(Collector through ElectionCollector, order by collectors::id.asc());
}
