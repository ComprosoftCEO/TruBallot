//
// All code related to error handling for the API server
//
mod client_request_error;
mod error_response;
mod global_error_codes;
mod shared_error;
mod websocket_error;

pub use client_request_error::ClientRequestError;
pub use error_response::ErrorResponse;
pub use global_error_codes::GlobalErrorCode;
pub(crate) use shared_error::SharedError;
pub use websocket_error::WebsocketError;
