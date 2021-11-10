use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use crate::auth::{audience, Audience, JWTClientData, JWTSecret, JWTToken, JWT_ISSUER};
use crate::errors::ServiceError;
use crate::notifications::WS_PROTOCOL;

/// Type aliases for the different JWT websocket tokens
pub type WebsocketToken = JWTWebsocketToken<audience::ClientOnly, JWTClientData>;

/// Special JWT token that deserializes from the 'Sec-WebSocket-Protocol' header
#[derive(Deserialize)]
pub struct JWTWebsocketToken<A: Audience, T>(JWTToken<A, T>);

impl<A: Audience, T> JWTWebsocketToken<A, T> {
  pub fn into_inner(self) -> JWTToken<A, T> {
    self.0
  }

  pub fn get_ref(&self) -> &JWTToken<A, T> {
    &self.0
  }
}

impl<A: Audience, T> Deref for JWTWebsocketToken<A, T> {
  type Target = JWTToken<A, T>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<A: Audience, T> DerefMut for JWTWebsocketToken<A, T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

//
// Get the JSON Web Token from the request
//
impl<A, T> FromRequest for JWTWebsocketToken<A, T>
where
  A: Audience + DeserializeOwned,
  T: Serialize + DeserializeOwned,
{
  type Error = ServiceError;
  type Future = Ready<Result<Self, ServiceError>>;
  type Config = ();

  fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
    let result: Result<Self, ServiceError> = (|| {
      // Get the list of all websocket protocols
      let req_protocols = match req.headers().get("Sec-WebSocket-Protocol") {
        None => return Err(ServiceError::MissingWebsocketJWT),
        Some(token) => token,
      }
      .to_str()
      .or_else(|e| Err(ServiceError::WebsocketJWTParseError(e)))?;

      // The bearer token will be the longest protocol that isn't the WS_PROTOCOL string
      let bearer_token = req_protocols
        .split(',')
        .map(|protocol| protocol.trim())
        .filter(|protocol| protocol != &WS_PROTOCOL)
        .max_by_key(|protocol| protocol.len())
        .ok_or_else(|| ServiceError::MissingWebsocketJWT)?;

      // Get the enccryption key from the app data
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
      let token_data = decode::<Self>(bearer_token, &jwt_secret.get_decoding_key(), &validation)?;
      Ok(token_data.claims)
    })();

    ready(result)
  }
}
