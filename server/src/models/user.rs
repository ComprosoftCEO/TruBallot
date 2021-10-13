use diesel::prelude::*;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::schema::users;
use crate::utils::new_safe_uuid_v4;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub hashed_password: String,
  pub name: String,
  pub refresh_secret: String,
}

impl User {
  model_base!(order by users::name.asc());

  has_many!(Election, order by elections::name.asc());
  has_many!(Election through Registration, order by elections::name.asc(), registered_elections);
  has_many!(Registration);
  has_many!(Commitment);

  /// Create a new user that is ready to be inserted into the database
  ///
  /// Always converts the email to lowercase first
  /// Does NOT attempt to verify the password complexity first
  /// DOES hash the password for the user (Which is why it returns a result)
  pub fn new(
    email: impl Into<String>,
    password: impl AsRef<[u8]>,
    name: impl Into<String>,
  ) -> Result<Self, ServiceError> {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    Ok(Self {
      id: new_safe_uuid_v4(),
      email: email.into().to_lowercase(),
      hashed_password,
      name: name.into(),
      refresh_secret: new_safe_uuid_v4().to_string(),
    })
  }

  ///
  /// Search for a user given their email address (which is unique)
  ///
  pub fn find_from_email(email: &str, conn: &DbConnection) -> Result<Self, ServiceError> {
    use crate::schema::users::dsl::{email as user_email, users};

    users
      .filter(user_email.eq(&email.to_lowercase()))
      .get_result::<Self>(conn.get())
      .optional()?
      .ok_or(ServiceError::InvalidEmailPassword)
  }

  ///
  /// Search for a user given their email address.
  ///  Returns "None" if the user is not found
  ///
  pub fn find_from_email_optional(email: &str, conn: &DbConnection) -> Result<Option<Self>, ServiceError> {
    use crate::schema::users::dsl::{email as user_email, users};

    Ok(
      users
        .filter(user_email.eq(&email.to_lowercase()))
        .get_result::<Self>(conn.get())
        .optional()?,
    )
  }

  /// Verify that the hashed password matches the input password
  pub fn verify_password(&self, password: impl AsRef<[u8]>) -> Result<bool, ServiceError> {
    Ok(bcrypt::verify(password, &self.hashed_password)?)
  }

  /// Update the user password
  pub fn update_password(&mut self, new_password: impl AsRef<[u8]>) -> Result<(), ServiceError> {
    let hashed_password = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)?;
    self.hashed_password = hashed_password;
    Ok(())
  }
}
