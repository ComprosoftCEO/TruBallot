//
// Structures and functions related to JSON web tokens
//
mod has_permission;
mod refresh_token;

pub use evoting_shared::jwt::*; // Re-export
pub use has_permission::HasPermission;
pub use refresh_token::RefreshToken;

// Other JWT constants
pub const JWT_REFRESH_AUDIENCE: &str = "refresh";
pub const JWT_REFRESH_EXPIRATION_MIN: i64 = 60;
