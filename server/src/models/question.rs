use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::{Commitment, Election};
use crate::schema::questions;
use crate::utils::new_safe_uuid_v4;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[belongs_to(Election)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Question {
  pub id: Uuid,
  pub election_id: Uuid,
  pub question: String,
  pub question_number: i64,

  pub final_forward_ballot: Option<BigDecimal>,
  pub final_reverse_ballot: Option<BigDecimal>,
  pub ballot_valid: bool,
}

impl Question {
  model_base!();

  belongs_to!(Election);
  has_many!(Commitment);
  has_many!(Candidate, order by candidates::candidate_number.asc());

  pub fn new(election_id: Uuid, question: impl Into<String>, question_number: i64) -> Self {
    Self {
      id: new_safe_uuid_v4(),
      election_id,
      question: question.into(),
      question_number,
      final_forward_ballot: None,
      final_reverse_ballot: None,
      ballot_valid: false,
    }
  }

  /// Search for election in the database, and return a ServiceError (not a Diesel error)
  pub fn find_resource(election_id: &Uuid, question_id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    let question = Self::find_optional(question_id, conn)?
      .ok_or_else(|| NamedResourceType::question(*election_id, *question_id).into_error())?;

    // Make sure the election ID matches
    if question.election_id != *election_id {
      Err(NamedResourceType::question(*election_id, *question_id).into_error())
    } else {
      Ok(question)
    }
  }

  /// Optionally get a commitment for a question
  pub fn find_commitment_optional(
    &self,
    user_id: &Uuid,
    conn: &DbConnection,
  ) -> Result<Option<Commitment>, ServiceError> {
    Ok(Commitment::find_optional(
      (user_id, &self.election_id, &self.id),
      &conn,
    )?)
  }
}
