use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use curv_kzen::{arithmetic::Modulo, BigInt};
use diesel::prelude::*;
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};
use uuid_b64::UuidB64 as Uuid;

use crate::auth::{ClientToken, JWTSecret, ServerToken, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::{ClientRequestError, ResourceAction, ServiceError};
use crate::models::{Candidate, Election, ElectionStatus, Question};
use crate::notifications::{notify_results_published, notify_voting_closed};
use crate::protocol::count_ballot_votes;
use crate::utils::ConvertBigInt;
use crate::Collector;

pub async fn close_voting(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;

  // Make sure the election exists
  let mut election = Election::find_resource(&*path, &conn)?;

  // Only the election creator can close voting
  let current_user_id = token.get_user_id();
  if election.created_by != current_user_id {
    return Err(ServiceError::ElectionNotOwnedByUser {
      current_user_id,
      owner_id: election.created_by,
      action: ResourceAction::CloseVoting,
    });
  }

  // Make sure the election is in the correct status
  if !(election.status == ElectionStatus::Voting || election.status == ElectionStatus::CollectionFailed) {
    return Err(ServiceError::WrongStatusFor {
      election_id: election.id,
      action: ResourceAction::CloseVoting,
      status: election.status,
    });
  }

  // Each question in the election MUST have at least 2 votes
  let mut questions: Vec<Question> = election.get_questions(&conn)?;
  for question in questions.iter() {
    if question.count_commitments(&conn)? < 2 {
      return Err(ServiceError::NotEnoughVotes {
        election_id: election.id,
        question_id: question.id,
      });
    }
  }

  // Mark the election as being closed
  election.status = ElectionStatus::CollectionFailed;
  election = election.update(&conn)?;
  notify_voting_closed(&election, &jwt_key).await;

  let jwt_encoding_key = jwt_key.get_encoding_key();
  let modulus = election.prime.to_bigint() - 1;
  let num_voters = election.count_registrations(&conn)?;

  // Cache all of the updates
  let mut all_candidates = Vec::new();
  for question in questions.iter_mut() {
    // Sum the ballots for each question
    let (forward_ballots, reverse_ballots) = question.get_ballots_sum(&modulus, &conn)?;

    // Cancel out any users who didn't vote
    let no_vote = question.get_user_ids_without_vote(&conn)?;
    let (forward_cancelation_shares, reverse_cancelation_shares) =
      get_cancelation_shares(&question, no_vote.clone(), &modulus, &jwt_encoding_key).await?;

    // Count the number of votes for each candidates
    //   (This process tests to make sure the voting vector is valid)
    let num_candidates = question.count_candidates(&conn)?;
    let num_votes = count_ballot_votes(
      &forward_ballots,
      &reverse_ballots,
      num_candidates,
      num_voters,
      no_vote.len(),
    );

    // Update the values in the database model
    //  Don't save yet, we will perform a massive transaction at the end
    question.final_forward_ballots =
      BigInt::mod_add(&forward_ballots, &forward_cancelation_shares, &modulus).to_bigdecimal();
    question.final_reverse_ballots =
      BigInt::mod_add(&reverse_ballots, &reverse_cancelation_shares, &modulus).to_bigdecimal();
    question.ballots_valid = num_votes.is_some();

    question.users_without_vote = no_vote;
    question.forward_cancelation_shares = forward_cancelation_shares.to_bigdecimal();
    question.reverse_cancelation_shares = reverse_cancelation_shares.to_bigdecimal();

    // Mark the candidates for update if the voting vector is valid
    //  Don't save yet, we will perform a massive transaction at the end
    if let Some(num_votes) = num_votes {
      let mut candidates: Vec<Candidate> = question.get_candidates(&conn)?;
      for candidate in candidates.iter_mut() {
        candidate.num_votes = Some(num_votes[candidate.candidate_number as usize]);
      }

      all_candidates.append(&mut candidates);
    }
  }

  // Perform a massive database transaction to update all questions at once
  election = conn.get().transaction::<_, ServiceError, _>(|| {
    for question in questions {
      question.update(&conn)?;
    }

    for candidate in all_candidates {
      candidate.update(&conn)?;
    }

    election.status = ElectionStatus::Finished;
    Ok(election.update(&conn)?)
  })?;

  notify_results_published(&election, &jwt_key).await;
  log::info!("Results published for election \"{}\" <{}>", election.name, election.id);

  Ok(HttpResponse::Ok().finish())
}

///
/// Send requests to the two collectors to get the cancelation shares
///
async fn get_cancelation_shares(
  question: &Question,
  user_ids: Vec<Uuid>,
  modulus: &BigInt,
  jwt_encoding_key: &EncodingKey,
) -> Result<(BigInt, BigInt), ServiceError> {
  // Make sure we actually need to get cancelation shares
  log::debug!("Get cancelation shares for question {}", question.question_number + 1);
  if user_ids.len() == 0 {
    log::debug!("All users voted, don't do anything...");
    return Ok((BigInt::from(0), BigInt::from(0)));
  }

  let cancelation_shares_data = CancelationSharesData { user_ids };
  let url = format!(
    "/elections/{}/questions/{}/cancelation",
    question.election_id, question.id
  );

  // Collector 1
  log::debug!("Request cancelation shares from collector 1...");
  let c1_shares_request = Client::builder()
    .disable_timeout()
    .bearer_auth(ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
    .finish()
    .post(Collector::One.api_url(&url)?)
    .send_json(&cancelation_shares_data);

  let c1_shares: CancelationShares = ClientRequestError::handle(c1_shares_request)
    .await
    .map_err(|e| ServiceError::CancelationSharesError(Collector::One, e))?;
  log::debug!("Success! Got cancelation shares from collector 1");

  // Collector 2
  log::debug!("Request cancelation shares from collector 2...");
  let c2_shares_request = Client::builder()
    .disable_timeout()
    .bearer_auth(ServerToken::new(DEFAULT_PERMISSIONS).encode(&jwt_encoding_key)?)
    .finish()
    .post(Collector::Two.api_url(&url)?)
    .send_json(&cancelation_shares_data);

  let c2_shares: CancelationShares = ClientRequestError::handle(c2_shares_request)
    .await
    .map_err(|e| ServiceError::CancelationSharesError(Collector::One, e))?;
  log::debug!("Success! Got cancelation shares from collector 2");

  // Compute the result
  Ok((
    BigInt::mod_add(
      &c1_shares.forward_cancelation_shares,
      &c2_shares.forward_cancelation_shares,
      modulus,
    ),
    BigInt::mod_add(
      &c1_shares.reverse_cancelation_shares,
      &c2_shares.reverse_cancelation_shares,
      modulus,
    ),
  ))
}

///
/// JSON structure to send to the collectors to get cancelation shares
///
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CancelationSharesData {
  user_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CancelationShares {
  #[serde(with = "kzen_paillier::serialize::bigint")]
  forward_cancelation_shares: BigInt,

  #[serde(with = "kzen_paillier::serialize::bigint")]
  reverse_cancelation_shares: BigInt,
}