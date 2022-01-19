// Macros from external libraries
#[macro_use]
extern crate diesel;
#[allow(unused)]
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate evoting_shared;

// All internal code modules
pub mod config;
pub mod errors;
pub mod handlers;
pub mod jwt;
pub mod models;
pub mod schema;
pub mod views;

// Re-export modules from the shared library
pub mod db {
  pub use evoting_shared::db::*;
}
pub mod protocol {
  pub use evoting_shared::protocol::*;
}
pub mod utils {
  pub use evoting_shared::utils::*;
}
