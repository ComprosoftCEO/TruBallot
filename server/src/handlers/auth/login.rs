use actix_web::{web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

use crate::auth::{verify_recaptcha, ClientToken, JWTSecret, Permission, RefreshToken};
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::User;
use crate::views::auth::LoginResult;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
  #[validate(email, length(min = 1, max = 255))]
  pub email: String,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  pub captcha: String,
}

pub async fn login(
  data: web::Json<LoginData>,
  conn: DbConnection,
  secret: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  data.validate()?;

  let LoginData {
    email,
    password,
    captcha,
  } = data.into_inner();

  verify_recaptcha(&captcha).await?;

  // Email is guaranteed to be unique by our index
  let user = User::find_from_email_optional(&email, &conn)?;

  // Always hash the password, even if the user doesn't exist
  //   This helps prevent timing attaks for our login form
  let password_valid = if let Some(ref user) = user {
    user.verify_password(password)?
  } else {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    false
  };

  if !password_valid {
    return Err(ServiceError::InvalidEmailPassword);
  }

  // Generate the JWT tokens
  let user = user.unwrap(); // Will not fail
  let permissions = [Permission::CanLogin, Permission::CreateElection, Permission::Vote];
  let refresh_token = RefreshToken::new(user.id);
  let client_token = ClientToken::new(user, &permissions);
  let encoding_key = secret.get_encoding_key();

  Ok(HttpResponse::Ok().json(LoginResult {
    client_token: client_token.encode(&encoding_key)?,
    refresh_token: refresh_token.encode(&encoding_key)?,
  }))
}
