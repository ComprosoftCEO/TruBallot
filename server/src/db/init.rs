use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{pg::PgConnection, Connection};

use crate::config;
use crate::db::{DbConnection, PgPool};

pub fn establish_new_connection_pool() -> anyhow::Result<PgPool> {
  let database_url =
    config::get_database_url().ok_or_else(|| anyhow::anyhow!("DATABASE_URL environment variable not set"))?;

  Ok(init_pool(&database_url)?)
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Pool::builder().build(manager)
}

pub fn open_new_connection() -> anyhow::Result<DbConnection> {
  let database_url =
    config::get_database_url().ok_or_else(|| anyhow::anyhow!("DATABASE_URL environment variable not set"))?;

  Ok(DbConnection::new(PgConnection::establish(&database_url)?))
}
