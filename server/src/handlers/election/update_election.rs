use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use super::helpers::validate_candidates;
use crate::db::DbConnection;
use crate::errors::{ResourceAction, ServiceError};
use crate::jwt::{ClientToken, HasPermission, JWTSecret};
use crate::models::{Candidate, Election, ElectionStatus, Question};
use crate::notifications::notify_election_updated;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateElectionData {
  #[validate(length(min = 1, max = 255))]
  pub name: Option<String>,
  pub is_public: Option<bool>,

  #[validate(length(min = 1))]
  #[validate]
  pub questions: Option<Vec<ElectionQuestion>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ElectionQuestion {
  #[validate(length(min = 1, max = 255))]
  pub name: String,

  #[validate(length(min = 2), custom = "validate_candidates")]
  pub candidates: Vec<String>,
}

pub async fn update_election(
  token: ClientToken,
  path: web::Path<Uuid>,
  data: web::Json<UpdateElectionData>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;
  data.validate()?;

  let UpdateElectionData {
    name,
    is_public,
    questions,
  } = data.into_inner();

  // Make sure the election exists
  let mut election = Election::find_resource(&*path, &conn)?;

  // Only the election creator can update the election
  let current_user_id = token.get_user_id();
  if election.created_by != current_user_id {
    return Err(ServiceError::ElectionNotOwnedByUser {
      current_user_id,
      owner_id: election.created_by,
      action: ResourceAction::Update,
    });
  }

  // Make sure the election is still a draft
  if election.status != ElectionStatus::Draft {
    return Err(ServiceError::ElectionNotDraft {
      election_id: election.id,
      action: ResourceAction::Update,
    });
  }

  // Update the election, questions, and candidates
  let election = conn.get().transaction::<_, ServiceError, _>(|| {
    // Possibly update the election parameters
    if let Some(name) = name {
      election.name = name;
    }
    if let Some(is_public) = is_public {
      election.is_public = is_public;
    }

    election = election.update(&conn)?;

    // Possibly update the questions
    if let Some(questions) = questions {
      // Delete and re-create the questions
      election.delete_all_questions(&conn)?;
      for (question_number, ElectionQuestion { name, candidates }) in questions.into_iter().enumerate() {
        let question = Question::new(election.id, name, question_number as i64).insert(&conn)?;

        for (candidate_number, candidate) in candidates.into_iter().enumerate() {
          Candidate::new(question.id, candidate, candidate_number as i64).insert(&conn)?;
        }
      }
    }

    Ok(election)
  })?;

  notify_election_updated(&election, &jwt_key).await;
  log::info!("Updated election \"{}\" <{}>", election.name, election.id);

  Ok(HttpResponse::Ok().finish())
}
