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
use crate::errors::{ResourceAction, ResourceType, ServiceError};

// Type aliases for the different JWT tokens
pub type ClientToken = JWTToken<audience::ClientOnly, JWTClientData>;
pub type ServerToken = JWTToken<audience::ServerOnly, JWTInternalData>;
pub type CollectorToken = JWTToken<audience::CollectorOnly, JWTInternalData>;
pub type MediatorToken = JWTToken<audience::MediatorOnly, JWTInternalData>;
pub type AnyToken = JWTToken<audience::All, JWTAnyData>;

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
  #[serde(flatten)]
  user_data: T,
  permissions: HashSet<Permission>,

  #[serde(skip)]
  _aud: PhantomData<A>,
}

/// Internal data used by the client JSON Web Token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTClientData {
  email: String,
  name: String,
}

pub type JWTInternalData = ();

/// Internal data used by the "Any" JSON Web Token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTAnyData {
  email: Option<String>,
  name: Option<String>,
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

  /// Test if the user has permission to do something
  pub fn has_permission(&self, p: Permission) -> bool {
    self.permissions.contains(&p)
  }

  //
  // Methods to test user permissions
  //
  pub fn test_can_manage_account(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::User,
        ResourceAction::Update,
      ))
    }
  }
  pub fn test_can_view_elections(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::ReadPrivate,
      ))
    }
  }

  pub fn test_can_create_election(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) && self.has_permission(Permission::CreateElection) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::Create,
      ))
    }
  }

  pub fn test_can_register_for_election(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) && self.has_permission(Permission::Register) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::Register,
      ))
    }
  }

  pub fn test_can_vote(&self) -> Result<(), ServiceError> {
    if self.has_permission(Permission::CanLogin) && self.has_permission(Permission::Vote) {
      Ok(())
    } else {
      Err(ServiceError::ForbiddenResourceAction(
        ResourceType::Election,
        ResourceAction::Register,
      ))
    }
  }
}

impl<A> JWTToken<A, JWTClientData>
where
  A: Audience,
{
  pub fn get_name(&self) -> &String {
    &self.user_data.name
  }

  pub fn get_email(&self) -> &String {
    &self.user_data.email
  }
}

impl<A> JWTToken<A, JWTInternalData>
where
  A: Audience,
{
  pub fn new(permissions: &[Permission]) -> Self {
    use uuid::Uuid;

    let now = Utc::now();
    let expiration = now + Duration::minutes(JWT_EXPIRATION_MIN);

    Self {
      iss: JWT_ISSUER.to_string(),
      sub: Uuid::nil().into(),
      aud: A::get_name(),
      iat: now.timestamp(),
      exp: expiration.timestamp(),

      user_data: (),
      permissions: HashSet::from_iter(permissions.into_iter().cloned()),

      _aud: PhantomData,
    }
  }
}

impl<A> JWTToken<A, JWTAnyData>
where
  A: Audience,
{
  pub fn get_name(&self) -> Option<&String> {
    self.user_data.name.as_ref()
  }

  pub fn get_email(&self) -> Option<&String> {
    self.user_data.email.as_ref()
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
        leeway: 15,
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
