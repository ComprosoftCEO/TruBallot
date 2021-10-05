use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::models::{Election, Question, User};
use crate::schema::commitments;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[primary_key(user_id, election_id, question_id)]
#[belongs_to(User)]
#[belongs_to(Question)]
#[belongs_to(Election)]
#[serde(rename_all = "camelCase")]
pub struct Commitment {
  pub user_id: Uuid,
  pub election_id: Uuid,
  pub question_id: Uuid,

  pub forward_ballot: BigDecimal,
  pub reverse_ballot: BigDecimal,

  pub g_s: BigDecimal,
  pub g_s_prime: BigDecimal,
  pub g_s_s_prime: BigDecimal,

  pub single_vote_verified: bool,
  pub published_ballots_verified: bool,
}

impl Commitment {
  model_base!();

  belongs_to!(User);
  belongs_to!(Election);
  belongs_to!(Question);

  // Sadly, our ORM can't represent this relationship
  // belongs_to!(Registration);
}
