use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
  pub client_token: String,
  pub refresh_token: String,
}
