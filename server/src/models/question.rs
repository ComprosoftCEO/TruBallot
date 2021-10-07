use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::Election;
use crate::schema::questions;
use crate::utils::new_safe_uuid_v4;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[belongs_to(Election)]
#[serde(rename_all = "camelCase")]
pub struct Question {
  pub id: Uuid,
  pub election_id: Uuid,
  pub question: String,
  pub question_number: i64,

  pub prime: BigDecimal,
  pub generator: BigDecimal,

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
      prime: BigDecimal::default(),
      generator: BigDecimal::default(),
      final_forward_ballot: None,
      final_reverse_ballot: None,
      ballot_valid: false,
    }
  }
}
