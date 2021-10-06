use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use diesel::pg::PgConnection;
use futures::future::{ready, Ready};
use std::ops::{Deref, DerefMut};

use crate::db::{PgPool, PgPooledConnection};
use crate::errors::ServiceError;

pub enum DbConnection {
  UnpooledConnection(PgConnection),
  PooledConnection(PgPooledConnection),
}

//
// Explicit extraction calls for when compiler can't implicitly convert
//
impl DbConnection {
  pub fn new(conn: PgConnection) -> Self {
    DbConnection::UnpooledConnection(conn)
  }
  pub fn new_pooled(conn: PgPooledConnection) -> Self {
    DbConnection::PooledConnection(conn)
  }

  pub fn get(&self) -> &PgConnection {
    match self {
      DbConnection::PooledConnection(pool) => &pool,
      DbConnection::UnpooledConnection(conn) => &conn,
    }
  }

  pub fn get_mut(&mut self) -> &mut PgConnection {
    match self {
      DbConnection::PooledConnection(ref mut pool) => pool,
      DbConnection::UnpooledConnection(ref mut conn) => conn,
    }
  }
}

//
// Implicitly convert to the database connection type
//
impl Deref for DbConnection {
  type Target = PgConnection;
  fn deref(&self) -> &Self::Target {
    self.get()
  }
}

impl DerefMut for DbConnection {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.get_mut()
  }
}

//
// Get the database connection from the request
//
impl FromRequest for DbConnection {
  type Error = Error;
  type Future = Ready<Result<DbConnection, Error>>;
  type Config = ();

  fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
    let res = get_connection_from_request(req);
    ready(res.map_err(|e| e.into()))
  }
}

pub fn get_connection_from_request(req: &HttpRequest) -> Result<DbConnection, ServiceError> {
  let pool = req
    .app_data::<web::Data<PgPool>>()
    .ok_or(ServiceError::MissingAppData("PgPool".into()))?;

  let conn = pool.get()?;
  Ok(DbConnection::new_pooled(conn))
}
