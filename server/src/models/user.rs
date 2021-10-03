use diesel::prelude::*;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub hashed_password: String,
  pub name: String,
}

impl User {
  model_base!();

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
}
