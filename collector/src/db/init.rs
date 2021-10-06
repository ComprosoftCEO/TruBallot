use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{pg::PgConnection, Connection};

use crate::config;
use crate::db::{DbConnection, PgPool};
use crate::errors::ServiceError;

pub fn establish_new_connection_pool() -> Result<PgPool, ServiceError> {
  let database_url =
    config::get_database_url().ok_or_else(|| ServiceError::MissingDatabaseConnectionUrl(config::get_collector()))?;
  Ok(init_pool(&database_url)?)
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Pool::builder().build(manager)
}

pub fn open_new_connection() -> Result<DbConnection, ServiceError> {
  let database_url =
    config::get_database_url().ok_or_else(|| ServiceError::MissingDatabaseConnectionUrl(config::get_collector()))?;
  Ok(DbConnection::new(PgConnection::establish(&database_url)?))
}
