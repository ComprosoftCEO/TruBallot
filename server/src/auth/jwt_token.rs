use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{offset::Utc, Duration};
use futures::executor::block_on;
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter::FromIterator;
use uuid_b64::UuidB64 as Uuid;

use crate::auth::{Audience, JWTSecret, Permission, JWT_EXPIRATION_MIN, JWT_ISSUER};
use crate::errors::ServiceError;
use crate::models::User;

/// JSON Web Token used for user authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWTToken {
  // Reserved Claims
  iss: String,   // Issuer
  sub: Uuid,     // Subject (whom token refers to)
  aud: Audience, // Audience (whom the token is intended for)
  iat: i64,      // Issued at (as UTC timestamp)
  exp: i64,      // Expiration time (as UTC timestamp)

  // Public and private claims
  email: String,
  name: String,
  user_data: JWTUserData,
}

/// Internal data used by the JSON Web Token
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JWTUserData {
  permissions: HashSet<Permission>,
}

impl JWTToken {
  /// Create a new JSON Web Token given the user, audience, and list of permissions
  pub fn new(user: User, audience: Audience, permissions: &[Permission]) -> Self {
    let now = Utc::now();
    let expiration = now + Duration::minutes(JWT_EXPIRATION_MIN);

    Self {
      iss: JWT_ISSUER.to_string(),
      sub: user.id,
      aud: audience,
      iat: now.timestamp(),
      exp: expiration.timestamp(),

      email: user.email,
      name: user.name,
      user_data: JWTUserData {
        permissions: HashSet::from_iter(permissions.into_iter().cloned()),
      },
    }
  }

  /// Encode the JSON Web Token into a string
  pub fn encode(&self, key: &EncodingKey) -> Result<String, ServiceError> {
    Ok(encode(&Header::default(), self, key)?)
  }

  pub fn get_user_id(&self) -> Uuid {
    self.sub
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn get_email(&self) -> &String {
    &self.email
  }

  /// Test if the user has permission to do something
  pub fn has_permission(&self, p: Permission) -> bool {
    self.user_data.permissions.contains(&p)
  }
}

//
// Get the JSON Web Token from the request
//
impl FromRequest for JWTToken {
  type Error = ServiceError;
  type Future = Ready<Result<Self, ServiceError>>;
  type Config = ();

  fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
    let result: Result<Self, ServiceError> = (|| {
      // Extract the JWT from the header and the encryption key from the app data
      let bearer_token = block_on(BearerAuth::from_request(req, pl))?;
      let jwt_secret = req.app_data::<web::Data<JWTSecret>>().expect("JWTSecret should be set");

      // Validation parameters,
      let validation = Validation {
        validate_exp: true,
        aud: Some(HashSet::from_iter([Audience::All.to_string()])),
        iss: Some(JWT_ISSUER.into()),
        ..Default::default()
      };

      // Decode and validate the JWT
      let token_data = decode::<Self>(bearer_token.token(), &jwt_secret.get_decoding_key(), &validation)?;
      Ok(token_data.claims)
    })();

    ready(result)
  }
}
