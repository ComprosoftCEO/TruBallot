use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::{Election, Question};
use crate::schema::registrations;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[primary_key(user_id, election_id, question_id)]
#[belongs_to(Election)]
#[belongs_to(Question)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Registration {
  pub user_id: Uuid,
  pub election_id: Uuid,
  pub question_id: Uuid,

  pub forward_verification_shares: BigDecimal,
  pub reverse_verification_shares: BigDecimal,

  pub forward_ballot_shares: BigDecimal,
  pub reverse_ballot_shares: BigDecimal,
}

impl Registration {
  model_base!();

  belongs_to!(Election);
  belongs_to!(Question);
}
