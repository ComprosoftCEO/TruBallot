use bigdecimal::BigDecimal;
use curv_kzen::BigInt;
use diesel::prelude::*;
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::{Commitment, Election, User};
use crate::schema::questions;
use crate::utils::{new_safe_uuid_v4, ConvertBigInt};

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[belongs_to(Election)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Question {
  pub id: Uuid,
  pub election_id: Uuid,
  pub question: String,
  pub question_number: i64,

  // Set to 0 until after the election has closed
  pub forward_cancelation_shares: BigDecimal,
  pub reverse_cancelation_shares: BigDecimal,
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
      forward_cancelation_shares: BigDecimal::default(),
      reverse_cancelation_shares: BigDecimal::default(),
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

  ///
  /// Get all (commitment, user) pairs for the given question
  ///
  pub fn get_commitments_users(&self, conn: &DbConnection) -> Result<Vec<(Commitment, User)>, ServiceError> {
    use crate::schema::commitments::dsl::{commitments, election_id, question_id};
    use crate::schema::users::dsl::{name, users};

    Ok(
      commitments
        .inner_join(users)
        .filter(election_id.eq(&self.election_id))
        .filter(question_id.eq(&self.id))
        .order_by(name.asc())
        .get_results(conn.get())?,
    )
  }

  ///
  /// Compute the forward and reverse ballot sum for a given question, mod (p - 1)
  ///   This method also applies the cancelation shares
  ///
  pub fn get_ballots_sum(&self, modulo: &BigInt, conn: &DbConnection) -> Result<(BigInt, BigInt), ServiceError> {
    use crate::schema::commitments::dsl::{commitments, election_id, forward_ballot, question_id, reverse_ballot};
    use diesel::dsl::sum;

    let forward: BigDecimal = commitments
      .select(sum(forward_ballot))
      .filter(election_id.eq(&self.election_id))
      .filter(question_id.eq(&self.id))
      .get_result::<Option<BigDecimal>>(conn.get())?
      .unwrap_or_else(BigDecimal::default);

    let reverse: BigDecimal = commitments
      .select(sum(reverse_ballot))
      .filter(election_id.eq(&self.election_id))
      .filter(question_id.eq(&self.id))
      .get_result::<Option<BigDecimal>>(conn.get())?
      .unwrap_or_else(BigDecimal::default);

    Ok((
      (forward + &self.forward_cancelation_shares).to_bigint() % modulo,
      (reverse + &self.reverse_cancelation_shares).to_bigint() % modulo,
    ))
  }

  ///
  /// Get the list of users who didn't cast a vote for this question
  ///
  pub fn get_users_without_vote_ordered(&self, conn: &DbConnection) -> Result<Vec<User>, ServiceError> {
    use crate::schema::commitments::dsl::{
      commitments, election_id as c_election_id, question_id as c_question_id, user_id as c_user_id,
    };
    use crate::schema::registrations::dsl::{election_id, registrations, user_id};
    use crate::schema::users::{
      all_columns as all_user_columns,
      dsl::{name, users},
    };
    use diesel::dsl::{exists, not};

    Ok(
      registrations
        .inner_join(users)
        .select(all_user_columns)
        .distinct()
        .filter(election_id.eq(&self.election_id))
        .filter(not(exists(
          commitments
            .filter(c_user_id.eq(user_id))
            .filter(c_election_id.eq(&self.election_id))
            .filter(c_question_id.eq(&self.id)),
        )))
        .order_by(name.asc())
        .get_results::<User>(conn.get())?,
    )
  }

  ///
  /// Get the list of user ids who didn't cast a vote for this question
  ///
  pub fn get_user_ids_without_vote(&self, conn: &DbConnection) -> Result<Vec<Uuid>, ServiceError> {
    Ok(
      self
        .get_users_without_vote_ordered(conn)?
        .into_iter()
        .map(|u| u.id)
        .collect(),
    )
  }
}
