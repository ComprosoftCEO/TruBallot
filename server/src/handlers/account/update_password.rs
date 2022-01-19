use actix_web::{web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

use crate::auth::validate_password_complexity;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{ClientToken, HasPermission};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePasswordData {
  #[validate(length(min = 1, max = 255))]
  pub current_password: String,

  #[validate(length(min = 1, max = 255))]
  pub new_password: String,
}

pub async fn update_password(
  token: ClientToken,
  data: web::Json<UpdatePasswordData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_manage_account()?;
  let mut user = token.validate_user_id(&conn)?;
  data.validate()?;

  let UpdatePasswordData {
    current_password,
    new_password,
  } = data.into_inner();

  // Make sure password complexity is okay
  validate_password_complexity(&new_password, &user.name, &user.email)?;

  // Test to ensure the current password matches
  if !user.verify_password(&current_password)? {
    return Err(ServiceError::InvalidEmailPassword);
  }

  // Okay, update the password
  user.update_password(new_password)?;
  user.update(&conn)?;

  log::info!("Updated password for user {} <{}>", token.get_name(), token.get_email());

  Ok(HttpResponse::Ok().finish())
}
