use actix_web::HttpResponse;

use crate::auth::AnyToken;
use crate::views::auth::UserDetails;

pub async fn get_me(token: AnyToken) -> HttpResponse {
  HttpResponse::Ok().json(UserDetails {
    id: token.get_user_id(),
    name: token.get_name().cloned(),
    email: token.get_email().cloned(),
  })
}
