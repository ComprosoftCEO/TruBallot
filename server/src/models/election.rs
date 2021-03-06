use bigdecimal::BigDecimal;
use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::iter;
use uuid_b64::UuidB64 as Uuid;

use crate::db::DbConnection;
use crate::errors::{NamedResourceType, ServiceError};
use crate::models::{Candidate, ElectionStatus, Question, Registration, User};
use crate::schema::elections;
use crate::utils::new_safe_uuid_v4;

pub const ACCESS_CODE_LENGTH: usize = 6;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[belongs_to(User, foreign_key = "created_by")]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Election {
  pub id: Uuid,
  pub name: String,
  pub created_by: Uuid,
  pub status: ElectionStatus,

  pub is_public: bool,
  pub access_code: Option<String>,

  pub generator: BigDecimal,
  pub prime: BigDecimal,

  pub location_modulus: BigDecimal,
}

/// Status for the current "voted" status for a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum HasVotedStatus {
  No = 0,
  Partial,
  Yes,
}

impl Election {
  model_base!(order by elections::name.asc());

  belongs_to!(User);
  has_many!(User through Registration, order by users::name.asc(), registered_users);
  has_many!(Question, order by questions::question_number.asc());
  has_many!(Registration);
  has_many!(Commitment);

  pub fn new(name: impl Into<String>, created_by: Uuid, is_public: bool) -> Self {
    Self {
      id: new_safe_uuid_v4(),
      name: name.into(),
      created_by,
      status: ElectionStatus::Draft,
      is_public,
      access_code: None,
      generator: BigDecimal::default(),
      prime: BigDecimal::default(),
      location_modulus: BigDecimal::default(),
    }
  }

  /// Search for election in the database, and return a ServiceError (not a Diesel error)
  pub fn find_resource(id: &Uuid, conn: &DbConnection) -> Result<Self, ServiceError> {
    Self::find_optional(id, conn)?.ok_or_else(|| NamedResourceType::election(*id).into_error())
  }

  /// Search for election in the database by access code (which should be unique)
  pub fn find_access_code(code: &str, conn: &DbConnection) -> Result<Option<Self>, ServiceError> {
    use crate::schema::elections::dsl::{access_code, elections};

    Ok(
      elections
        .filter(access_code.eq(code))
        .get_result::<Self>(conn.get())
        .optional()?,
    )
  }

  /// Get the questions and the candidates, ordered by question number
  pub fn get_questions_candidates_ordered(
    &self,
    conn: &DbConnection,
  ) -> Result<Vec<(Question, Vec<Candidate>)>, ServiceError> {
    let questions = self.get_questions_ordered(conn)?;
    let candidates = Candidate::belonging_to(&questions)
      .get_results::<Candidate>(conn.get())?
      .grouped_by(&questions);

    Ok(questions.into_iter().zip(candidates).collect())
  }

  /// Find a question from this election given the question ID
  pub fn find_question(&self, question_id: &Uuid, conn: &DbConnection) -> Result<Question, ServiceError> {
    Question::find_resource(&self.id, question_id, conn)
  }

  /// Get a user registration for an election
  pub fn get_user_registration(
    &self,
    user_id: &Uuid,
    conn: &DbConnection,
  ) -> Result<Option<Registration>, ServiceError> {
    Ok(Registration::find_optional((&user_id, &self.id), conn)?)
  }

  /// Test if a user has voted for every question in the election
  ///    "No", "Partial", or "Yes"
  pub fn has_user_voted_status(&self, user_id: &Uuid, conn: &DbConnection) -> Result<HasVotedStatus, ServiceError> {
    let num_commitments = self.count_user_commitments(user_id, conn)?;
    if num_commitments == 0 {
      return Ok(HasVotedStatus::No);
    }

    let num_questions = self.count_questions(conn)?;
    if num_commitments < num_questions {
      Ok(HasVotedStatus::Partial)
    } else {
      Ok(HasVotedStatus::Yes)
    }
  }

  /// Count the number of commitments that a user has submitted
  fn count_user_commitments(&self, user_id: &Uuid, conn: &DbConnection) -> Result<i64, ServiceError> {
    use crate::schema::commitments::dsl::{commitments, election_id, user_id as commitment_user_id};

    Ok(
      commitments
        .filter(election_id.eq(&self.id))
        .filter(commitment_user_id.eq(user_id))
        .count()
        .get_result(conn.get())?,
    )
  }

  /// Test if a user is currently registered for an election
  pub fn is_user_registered(&self, user_id: &Uuid, conn: &DbConnection) -> Result<bool, ServiceError> {
    Ok(self.get_user_registration(user_id, conn)?.is_some())
  }

  /// Generate an access code, making sure the code doesn't alreay exist in the database
  pub fn generate_unique_access_code(&mut self, conn: &DbConnection) -> Result<(), ServiceError> {
    // Keep generating codes until we find a unique one
    //
    // With 36 characters (A-Z and 0-9), this gives us 2_176_782_336 or 2.1 billion codes
    // Since codes are cleared after the registration period ends, we only run into problems
    // if hundreds of millions of elections are open for registration simultaneously
    //
    // For our purposes, this should not be a problem realistically
    let mut rng = rand::thread_rng();
    let code = loop {
      // Generate a random access code (A-Z and 0-9)
      let code: String = iter::repeat(())
        .map(|_| char::from(rng.sample(Alphanumeric)).to_ascii_uppercase())
        .take(ACCESS_CODE_LENGTH)
        .collect();

      // Make sure no-one is currently using the code
      if Election::find_access_code(&code, &conn)?.is_none() {
        break code;
      }
    };

    self.access_code = Some(code);
    Ok(())
  }
}
