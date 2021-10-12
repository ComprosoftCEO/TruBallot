//
// All API handlers for registering for an election
//
mod open_registration;
mod register_for_election;
mod unregister_from_election;

pub use open_registration::open_registration;
pub use register_for_election::register_for_election;
pub use unregister_from_election::unregister_from_election;
