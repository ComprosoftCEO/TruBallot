use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
  pub id: Uuid,
  pub name: String,
  pub email: String,
}
