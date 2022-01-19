use actix_web::{web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

use crate::auth::{validate_password_complexity, verify_recaptcha};
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::JWTSecret;
use crate::models::User;
use crate::views::auth::LoginResult;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationData {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(email, length(min = 1, max = 255))]
  pub email: String,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  pub captcha: String,
}

pub async fn register_account(
  data: web::Json<RegistrationData>,
  conn: DbConnection,
  secret: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  data.validate()?;

  let RegistrationData {
    name,
    email,
    password,
    captcha,
  } = data.into_inner();

  verify_recaptcha(&captcha).await?;

  // Make sure email does not already exist
  if let Some(user) = User::find_from_email_optional(&email, &conn)? {
    return Err(ServiceError::UserEmailExists { email: user.email });
  }

  // Make sure password complexity is okay
  validate_password_complexity(&password, &name, &email)?;

  // We are good! Create the new user account!
  let user = User::new(email, password, name)?.insert(&conn)?;

  // Generate the JWT tokens
  let result = LoginResult::build(user, &*secret)?;

  log::info!(
    "Created new user {} <{}> in system and logging in",
    result.get_name(),
    result.get_email()
  );

  Ok(HttpResponse::Ok().json(result))
}
