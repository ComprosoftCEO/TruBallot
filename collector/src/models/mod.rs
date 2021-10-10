//
// Database tables (ORM models)
//
mod election;
mod question;
mod registration;

pub use election::Election;
pub use question::Question;
pub use registration::Registration;
