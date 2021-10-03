use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{offset::Utc, Duration};
use futures::executor::block_on;
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::marker::PhantomData;
use uuid_b64::UuidB64 as Uuid;

use crate::auth::{audience, Audience, JWTSecret, Permission, JWT_EXPIRATION_MIN, JWT_ISSUER};
use crate::errors::ServiceError;
use crate::models::User;

// Type aliases for the different JWT tokens
pub type ClientToken = JWTToken<audience::ClientOnly, JWTUserData>;
pub type ServerToken = JWTToken<audience::ServerOnly, JWTUserData>;
pub type CollectorToken = JWTToken<audience::CollectorOnly, JWTUserData>;

/// JSON Web Token used for user authentication
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTToken<A: Audience, T> {
  // Reserved Claims
  iss: String, // Issuer
  sub: Uuid,   // Subject (whom token refers to)
  aud: String, // Audience (whom the token is intended for)
  iat: i64,    // Issued at (as UTC timestamp)
  exp: i64,    // Expiration time (as UTC timestamp)

  // Public and private claims
  email: String,
  name: String,
  user_data: T,
  _aud: PhantomData<A>,
}

/// Internal data used by the JSON Web Token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTUserData {
  permissions: HashSet<Permission>,
}

impl<A, T> JWTToken<A, T>
where
  A: Audience,
  T: Serialize + DeserializeOwned,
{
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
}

impl<A> JWTToken<A, JWTUserData>
where
  A: Audience,
{
  /// Create a new JSON Web Token given the user, audience, and list of permissions
  pub fn new(user: User, permissions: &[Permission]) -> Self {
    let now = Utc::now();
    let expiration = now + Duration::minutes(JWT_EXPIRATION_MIN);

    Self {
      iss: JWT_ISSUER.to_string(),
      sub: user.id,
      aud: A::get_name(),
      iat: now.timestamp(),
      exp: expiration.timestamp(),

      email: user.email,
      name: user.name,
      user_data: JWTUserData {
        permissions: HashSet::from_iter(permissions.into_iter().cloned()),
      },
      _aud: PhantomData,
    }
  }

  /// Test if the user has permission to do something
  pub fn has_permission(&self, p: Permission) -> bool {
    self.user_data.permissions.contains(&p)
  }
}

//
// Get the JSON Web Token from the request
//
impl<A, T> FromRequest for JWTToken<A, T>
where
  A: Audience,
  T: Serialize + DeserializeOwned,
{
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
        aud: Some(A::accepts()),
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
