use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::auth::{ClientToken, JWTSecret, RefreshToken, DEFAULT_PERMISSIONS};
use crate::errors::ServiceError;
use crate::models::User;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
  pub client_token: String,
  pub refresh_token: String,

  // Helpful internal fields, not exposed externally
  #[serde(skip)]
  name: String,
  #[serde(skip)]
  email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
  pub id: Uuid,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
}

impl LoginResult {
  /// Helpful method to build the LoginResult given the user and JWT secret
  pub fn build(user: User, secret: &JWTSecret) -> Result<Self, ServiceError> {
    // Get the JWT keys
    let client_encoding_key = secret.get_encoding_key();
    let refresh_encoding_key = user.get_refresh_encoding_key();

    // Generate the JWT tokens
    let refresh_token = RefreshToken::new(user.id);
    let client_token = ClientToken::new(user, DEFAULT_PERMISSIONS);

    // Encode the tokens
    Ok(Self {
      client_token: client_token.encode(&client_encoding_key)?,
      refresh_token: refresh_token.encode(&refresh_encoding_key)?,

      name: client_token.get_name().clone(),
      email: client_token.get_email().clone(),
    })
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn get_email(&self) -> &String {
    &self.email
  }
}
