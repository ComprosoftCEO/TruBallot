use actix_web::{web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::{ClientToken, HasPermission, JWTSecret};
use crate::notifications::notify_name_changed;
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
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_manage_account()?;
  let mut user = token.validate_user_id(&conn)?;
  data.validate()?;

  let UpdateAccountData { name } = data.into_inner();

  // Update the account details
  if let Some(name) = name {
    user.name = name;
  }

  user = user.update(&conn)?;

  notify_name_changed(&user, &jwt_key).await;
  log::info!("Updated account details for {} <{}>", user.name, user.email);

  // Generate the JWT tokens
  let result = LoginResult::build(user, &jwt_key)?;
  Ok(HttpResponse::Ok().json(result))
}
