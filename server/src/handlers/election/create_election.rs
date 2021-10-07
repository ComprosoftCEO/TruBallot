use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::{Validate, ValidationError};

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::{Candidate, Election, Question};
use crate::views::election::NewElectionResult;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionData {
  #[validate(length(min = 1, max = 255))]
  pub name: String,

  #[validate(length(min = 1))]
  #[validate]
  pub questions: Vec<ElectionQuestion>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElectionQuestion {
  #[validate(length(min = 1, max = 255))]
  pub name: String,

  #[validate(length(min = 1), custom = "validate_candidates")]
  pub candidates: Vec<String>,
}

///
/// Custom validator function for the individual candidates
///   Checks to make sure the string has between 1 and 255 characters
///
fn validate_candidates(candidates: &Vec<String>) -> Result<(), ValidationError> {
  // Each candidate must have between 1 and 255 characters in the string
  for candidate in candidates {
    if candidate.len() < 1 || candidate.len() > 255 {
      return Err(ValidationError::new(
        "length [{\"max\": Number(255), \"value\": String(\"\"), \"min\": Number(1)}]",
      ));
    }
  }

  Ok(())
}

pub async fn create_election(
  token: ClientToken,
  data: web::Json<CreateElectionData>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;
  data.validate()?;

  // Create the election, questions, and candidates
  let CreateElectionData { name, questions } = data.into_inner();
  let new_id = conn.get().transaction::<Uuid, ServiceError, _>(|| {
    let election = Election::new(name, token.get_user_id()).insert(&conn)?;

    for (question_number, ElectionQuestion { name, candidates }) in questions.into_iter().enumerate() {
      let question = Question::new(election.id, name, question_number as i64).insert(&conn)?;

      for (candidate_number, candidate) in candidates.into_iter().enumerate() {
        Candidate::new(question.id, candidate, candidate_number as i64).insert(&conn)?;
      }
    }

    Ok(election.id)
  })?;

  Ok(HttpResponse::Ok().json(NewElectionResult { id: new_id }))
}
