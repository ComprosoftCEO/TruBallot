use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{pg::PgConnection, Connection};

use crate::config;
use crate::db::{DbConnection, PgPool};

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Pool::builder().build(manager)
}

pub fn establish_new_connection_pool() -> PgPool {
  let database_url = config::get_database_url().expect("DATABASE_URL must be set");
  init_pool(&database_url).expect("Failed to create pool")
}

pub fn open_new_connection() -> DbConnection {
  let database_url = config::get_database_url().expect("DATABASE_URL must be set");
  DbConnection::new(PgConnection::establish(&database_url).expect("Failed to connect to database"))
}
