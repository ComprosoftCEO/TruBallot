//
// All code related to error handling for the API server
//
mod service_error;

pub use evoting_shared::errors::*; // Re-export
pub use service_error::ServiceError;
