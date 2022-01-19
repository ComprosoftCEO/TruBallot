//
// Structures and functions related to JSON web tokens
//
pub mod audience;
mod jwt_secret;
mod jwt_token;

pub use crate::auth::{Permission, DEFAULT_PERMISSIONS}; // Re-export
pub use audience::Audience;
pub use jwt_secret::JWTSecret;
pub use jwt_token::*;

// Other JWT constants
pub const JWT_ISSUER: &str = "evoting";
pub const JWT_EXPIRATION_MIN: i64 = 10;
