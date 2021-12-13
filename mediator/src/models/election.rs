use bigdecimal::BigDecimal;
use curv_kzen::BigInt;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::{Question, Registration};
use crate::schema::elections;
use crate::utils::ConvertBigInt;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Election {
  pub id: Uuid,
  pub is_public: bool,
  pub generator: BigDecimal,
  pub prime: BigDecimal,
}

impl Election {
  model_base!();

  has_many!(Question);
  has_many!(Registration);
  has_many!(Collector through ElectionCollector, order by collectors::id.asc());

  pub fn new(id: Uuid, is_public: bool, generator: &BigInt, prime: &BigInt) -> Self {
    Self {
      id,
      is_public,
      generator: generator.to_bigdecimal(),
      prime: prime.to_bigdecimal(),
    }
  }

  pub fn find_resource(id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    Self::find_optional(id, conn)?.ok_or_else(|| NamedResourceType::election(*id).into_error())
  }

  /// Get a user registration for an election
  pub fn get_registration_optional(
    &self,
    user_id: &Uuid,
    conn: &DbConnection,
  ) -> Result<Option<Registration>, ServiceError> {
    Ok(Registration::find_optional((user_id, &self.id), conn)?)
  }

  /// Get a question for an election
  pub fn get_question(&self, question_id: &Uuid, conn: &DbConnection) -> Result<Question, ServiceError> {
    Question::find_resource(question_id, &self.id, conn)
  }
}
