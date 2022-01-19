use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{pg::PgConnection, Connection};

use crate::db::{DbConnection, PgPool};

pub fn open_new_connection_pool(database_url: &str) -> anyhow::Result<PgPool> {
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Ok(Pool::builder().build(manager)?)
}

pub fn open_new_connection(database_url: &str) -> anyhow::Result<DbConnection> {
  Ok(DbConnection::new(PgConnection::establish(&database_url)?))
}
