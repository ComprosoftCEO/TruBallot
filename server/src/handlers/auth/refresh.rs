use actix_web::{web, HttpResponse};

use crate::auth::{ClientToken, JWTSecret, RefreshToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::ServiceError;
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
    return Err(ServiceError::JWTNoSuchUser(token.get_user_id()));
  };

  // Generate the JWT tokens
  let refresh_token = RefreshToken::new(user.id);
  let client_token = ClientToken::new(user, DEFAULT_PERMISSIONS);

  // Encode the tokens
  let encoding_key = secret.get_encoding_key();
  let result = LoginResult {
    client_token: client_token.encode(&encoding_key)?,
    refresh_token: refresh_token.encode(&encoding_key)?,
  };

  Ok(HttpResponse::Ok().json(result))
}
