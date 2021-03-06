//
// All API handlers for managing the collectors in the database
//
mod all_collectors;
mod create_or_update_collector;
mod get_collector;
mod proxy_collector;
mod update_collector;

pub use all_collectors::all_collectors;
pub use create_or_update_collector::create_or_update_collector;
pub use get_collector::get_collector;
pub use proxy_collector::proxy_collector;
pub use update_collector::update_collector;
