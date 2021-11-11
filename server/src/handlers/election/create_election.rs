use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::helpers::validate_candidates;
use crate::auth::{ClientToken, JWTSecret};
use crate::db::DbConnection;
use crate::errors::ServiceError;
use crate::models::{Candidate, Election, Question};
use crate::notifications::notify_election_created;
use crate::views::election::NewElectionResult;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateElectionData {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  pub is_public: bool,

  #[validate(length(min = 1))]
  #[validate]
  pub questions: Vec<ElectionQuestion>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElectionQuestion {
  #[validate(length(min = 1, max = 255))]
  pub name: String,

  #[validate(length(min = 2), custom = "validate_candidates")]
  pub candidates: Vec<String>,
}

pub async fn create_election(
  token: ClientToken,
  data: web::Json<CreateElectionData>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;
  data.validate()?;

  let CreateElectionData {
    name,
    is_public,
    questions,
  } = data.into_inner();

  // Create the election, questions, and candidates
  let new_election = conn.get().transaction::<_, ServiceError, _>(|| {
    let election = Election::new(name, token.get_user_id(), is_public).insert(&conn)?;

    for (question_number, ElectionQuestion { name, candidates }) in questions.into_iter().enumerate() {
      let question = Question::new(election.id, name, question_number as i64).insert(&conn)?;

      for (candidate_number, candidate) in candidates.into_iter().enumerate() {
        Candidate::new(question.id, candidate, candidate_number as i64).insert(&conn)?;
      }
    }

    Ok(election)
  })?;

  notify_election_created(&new_election, new_election.created_by, &jwt_key).await;
  log::info!("New election created: \"{}\" <{}>", new_election.name, new_election.id);

  Ok(HttpResponse::Ok().json(NewElectionResult { id: new_election.id }))
}
