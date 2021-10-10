use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
  pub id: Uuid,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
}
