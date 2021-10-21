use bigdecimal::BigDecimal;
use curv_kzen::BigInt;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::Election;
use crate::schema::encrypted_locations;
use crate::utils::ConvertBigInt;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[primary_key(user_id, election_id)]
#[belongs_to(Election)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct EncryptedLocation {
  pub user_id: Uuid,
  pub election_id: Uuid,
  pub location: BigDecimal,
}

impl EncryptedLocation {
  model_base!();

  belongs_to!(Election);

  pub fn new(user_id: Uuid, election_id: Uuid, location: &BigInt) -> Self {
    Self {
      user_id,
      election_id,
      location: location.to_bigdecimal(),
    }
  }

  /// Search for the encrypted location in the database, and return a ServiceError (not a Diesel error)
  pub fn find_resource(user_id: &Uuid, election_id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    Self::find_optional((user_id, election_id), conn)?
      .ok_or_else(|| NamedResourceType::encrypted_location(*user_id, *election_id).into_error())
  }
}
