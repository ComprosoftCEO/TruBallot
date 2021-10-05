use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::Question;
use crate::schema::candidates;
use crate::utils::new_safe_uuid_v4;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[belongs_to(Question)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
  pub id: Uuid,
  pub question_id: Uuid,
  pub candidate: String,
  pub num_votes: Option<i64>,
}

impl Candidate {
  model_base!();

  belongs_to!(Question);

  pub fn new(question_id: Uuid, candidate: impl Into<String>) -> Self {
    Self {
      id: new_safe_uuid_v4(),
      question_id,
      candidate: candidate.into(),
      num_votes: None,
    }
  }
}
