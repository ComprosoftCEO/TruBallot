use actix_web::{web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

use crate::auth::{ClientToken, JWTSecret, RefreshToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::views::auth::LoginResult;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountData {
  #[validate(length(min = 1, max = 255))]
  pub name: Option<String>,
}

pub async fn update_account(
  token: ClientToken,
  data: web::Json<UpdateAccountData>,
  conn: DbConnection,
  secret: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_manage_account()?;
  let mut user = token.validate_user_id(&conn)?;
  data.validate()?;

  let UpdateAccountData { name } = data.into_inner();

  // Okay, update the account details
  if let Some(name) = name {
    user.name = name;
  }

  user.update(&conn)?;

  // Generate the JWT tokens since the name may have changed
  let refresh_token = RefreshToken::new(user.id);
  let client_token = ClientToken::new(user, DEFAULT_PERMISSIONS);

  // Encode the tokens
  let encoding_key = secret.get_encoding_key();
  let result = LoginResult {
    client_token: client_token.encode(&encoding_key)?,
    refresh_token: refresh_token.encode(&encoding_key)?,
  };

  log::info!(
    "Updated account details for {} <{}>",
    client_token.get_name(),
    client_token.get_email()
  );

  Ok(HttpResponse::Ok().json(result))
}
