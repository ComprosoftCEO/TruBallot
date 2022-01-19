use actix_web::{web, HttpResponse};

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::jwt::JWTSecret;
use crate::jwt::RefreshToken;
use crate::models::User;
use crate::views::auth::LoginResult;

pub async fn refresh(
  token: RefreshToken,
  conn: DbConnection,
  secret: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  let user = if let Some(user) = User::find_optional(&token.get_user_id(), &conn)? {
    user
  } else {
    return Err(ServiceError::JWTNoSuchUser {
      user_id: token.get_user_id(),
    });
  };

  // Generate the JWT tokens
  let result = LoginResult::build(user, &*secret)?;
  Ok(HttpResponse::Ok().json(result))
}
