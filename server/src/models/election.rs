use bigdecimal::BigDecimal;
use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;
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

  pub encryption_key: Vec<u8>,
}

impl Election {
  model_base!(order by elections::name.asc());

  belongs_to!(User);
  has_many!(User through Registration, order by users::name.asc(), registered_users);
  has_many!(Question, order by questions::question_number.asc());
  has_many!(Registration);
  has_many!(Commitment);

  pub fn new(name: impl Into<String>, created_by: Uuid, is_public: bool) -> Self {
    // Generate a random AES encryption key
    let encryption_key = thread_rng().gen::<[u8; 32]>().to_vec();

    Self {
      id: new_safe_uuid_v4(),
      name: name.into(),
      created_by,
      status: ElectionStatus::Draft,
      is_public,
      access_code: None,
      generator: BigDecimal::default(),
      prime: BigDecimal::default(),
      encryption_key,
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

  /// Get the questions and the candidates
  pub fn get_questions_candidates(&self, conn: &DbConnection) -> Result<Vec<(Question, Vec<Candidate>)>, ServiceError> {
    let questions = self.get_questions(conn)?;
    let candidates = Candidate::belonging_to(&questions)
      .get_results::<Candidate>(conn.get())?
      .grouped_by(&questions);

    Ok(questions.into_iter().zip(candidates).collect())
  }

  /// Get a user registration for an election
  pub fn get_user_registration(
    &self,
    user_id: &Uuid,
    conn: &DbConnection,
  ) -> Result<Option<Registration>, ServiceError> {
    Ok(Registration::find_optional((&user_id, &self.id), conn)?)
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
