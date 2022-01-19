use actix_web::{dev::Payload, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{offset::Utc, Duration};
use futures::executor::block_on;
use futures::future::{ready, Ready};
use jsonwebtoken::{dangerous_insecure_decode, decode, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter::FromIterator;
use uuid_b64::UuidB64 as Uuid;

use crate::db::get_connection_from_request;
use crate::errors::ServiceError;
use crate::jwt::{Permission, JWT_ISSUER, JWT_REFRESH_AUDIENCE, JWT_REFRESH_EXPIRATION_MIN};
use crate::models::User;

/// Special JWT token that is designed to refresh an existing token
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
  iss: String, // Issuer
  sub: Uuid,   // Subject (whom token refers to)
  aud: String, // Audience (whom the token is intended for)
  iat: i64,    // Issued at (as UTC timestamp)
  exp: i64,    // Expiration time (as UTC timestamp)

  name: String,
  email: String,
  permissions: HashSet<Permission>,
}

impl RefreshToken {
  /// Create a new refresh token
  pub fn new(user: User, permissions: &[Permission]) -> Self {
    let now = Utc::now();
    let expiration = now + Duration::minutes(JWT_REFRESH_EXPIRATION_MIN);

    Self {
      iss: JWT_ISSUER.to_string(),
      sub: user.id,
      aud: JWT_REFRESH_AUDIENCE.to_string(),
      iat: now.timestamp(),
      exp: expiration.timestamp(),

      name: user.name,
      email: user.email,
      permissions: HashSet::from_iter(permissions.into_iter().cloned()),
    }
  }

  /// Encode the JSON Web Token into a string
  pub fn encode(&self, key: &EncodingKey) -> Result<String, ServiceError> {
    Ok(encode(&Header::default(), self, key)?)
  }

  pub fn get_user_id(&self) -> Uuid {
    self.sub
  }
}

//
// Get the JSON Web Token from the request
//
impl FromRequest for RefreshToken {
  type Error = ServiceError;
  type Future = Ready<Result<Self, ServiceError>>;
  type Config = ();

  fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
    let result: Result<Self, ServiceError> = (|| {
      // Extract the JWT from the header
      let bearer_token = block_on(BearerAuth::from_request(req, pl))?;

      // Decode the JWT without any validation to get the user ID
      let insecure_token = dangerous_insecure_decode::<Self>(bearer_token.token())?;
      let user_id = insecure_token.claims.get_user_id();

      // Refresh secret key is specific to the user (Stored in the users table)
      let conn = get_connection_from_request(req)?;
      let user: User = User::find_optional(&user_id, &conn)?.ok_or_else(|| ServiceError::JWTNoSuchUser { user_id })?;
      let decoding_key = user.get_refresh_decoding_key();

      // Validation parameters,
      let validation = Validation {
        validate_exp: true,
        leeway: 15,
        aud: Some(HashSet::from_iter([JWT_REFRESH_AUDIENCE.into()])),
        iss: Some(JWT_ISSUER.into()),

        ..Default::default()
      };

      // Decode and validate the JWT
      let token_data = decode::<Self>(bearer_token.token(), &decoding_key, &validation)?;
      Ok(token_data.claims)
    })();

    ready(result)
  }
}
