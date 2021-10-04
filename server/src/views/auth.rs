use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
  pub client_token: String,
  pub refresh_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
  pub id: Uuid,
  pub name: String,
  pub email: String,
}
