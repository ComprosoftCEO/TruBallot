use bigdecimal::BigDecimal;
use curv_kzen::BigInt;
use kzen_paillier::DecryptionKey;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::{EncryptedLocation, Registration};
use crate::schema::elections;
use crate::utils::ConvertBigInt;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Election {
  pub id: Uuid,
  pub generator: BigDecimal,
  pub prime: BigDecimal,
  pub paillier_p: BigDecimal,
  pub paillier_q: BigDecimal,
}

impl Election {
  model_base!();

  has_many!(Registration);
  has_many!(EncryptedLocation);

  pub fn new(id: Uuid, generator: &BigInt, prime: &BigInt, paillier: &DecryptionKey) -> Self {
    // Convert from BigInt to BigDecimal
    let generator = generator.to_bigdecimal();
    let prime = prime.to_bigdecimal();

    let paillier_p = paillier.p.to_bigdecimal();
    let paillier_q = paillier.q.to_bigdecimal();

    Self {
      id,
      generator,
      prime,
      paillier_p,
      paillier_q,
    }
  }

  /// Search for election in the database, and return a ServiceError (not a Diesel error)
  pub fn find_resource(id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    Self::find_optional(id, conn)?.ok_or_else(|| NamedResourceType::election(*id).into_error())
  }

  /// Get a user registration for an election
  pub fn get_registration(
    &self,
    question_id: &Uuid,
    user_id: &Uuid,
    conn: &DbConnection,
  ) -> Result<Option<Registration>, ServiceError> {
    Ok(Registration::find_optional((user_id, &self.id, &question_id), conn)?)
  }

  /// Get the encrypted location for a given user
  pub fn get_user_encrypted_location(
    &self,
    user_id: &Uuid,
    conn: &DbConnection,
  ) -> Result<Option<EncryptedLocation>, ServiceError> {
    Ok(EncryptedLocation::find_optional((user_id, &self.id), conn)?)
  }
}
