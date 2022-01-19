use jsonwebtoken::{DecodingKey, EncodingKey};

/// Simple wrapper struct to store the JWT secret
#[derive(Debug, Clone)]
pub struct JWTSecret {
  secret: String,
}

impl JWTSecret {
  pub fn new(secret: impl Into<String>) -> Self {
    Self { secret: secret.into() }
  }

  pub fn get_encoding_key(&self) -> EncodingKey {
    EncodingKey::from_secret(self.secret.as_bytes())
  }

  pub fn get_decoding_key<'a>(&'a self) -> DecodingKey<'a> {
    DecodingKey::from_secret(self.secret.as_bytes())
  }
}
