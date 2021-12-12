use http::Uri;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::errors::ServiceError;
use crate::models::Collector;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicCollectorList {
  pub id: Uuid,
  pub name: String,

  #[serde(with = "http_serde::uri")]
  pub private_base_uri: Uri,

  pub is_secure: bool,
}

pub type PublicCollector = PublicCollectorList;

impl PublicCollectorList {
  pub fn from_collector(c: Collector) -> Result<Self, ServiceError> {
    let private_base_uri = c.get_private_base_uri()?;
    Ok(Self {
      id: c.id,
      name: c.name,
      private_base_uri,
      is_secure: c.is_secure,
    })
  }
}
