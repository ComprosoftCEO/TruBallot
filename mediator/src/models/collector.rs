use http::Uri;
use serde::Serialize;
use std::str::FromStr;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::schema::collectors;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Collector {
  pub id: Uuid,
  pub name: String,
  pub private_base_uri: String,
  pub is_secure: bool,
}

impl Collector {
  model_base!(order by collectors::name.asc());

  has_many!(Election through ElectionCollector);

  pub fn get_private_base_uri(&self) -> Result<Uri, ServiceError> {
    Uri::from_str(&self.private_base_uri).map_err(|e| ServiceError::InvalidCollectorURI(self.id, e))
  }

  pub fn find_resource(id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    Self::find_optional(id, conn)?.ok_or_else(|| NamedResourceType::collector(*id).into_error())
  }

  pub fn private_api_url(&self, path: &str) -> String {
    format!(
      "{}:{}/api/v1/collector{}",
      if self.is_secure { "https" } else { "http" },
      self.private_base_uri,
      path
    )
  }

  pub fn private_websocket_url(&self, path: &str) -> String {
    format!(
      "{}:{}/api/v1/collector{}",
      if self.is_secure { "wss" } else { "ws" },
      self.private_base_uri,
      path
    )
  }
}
