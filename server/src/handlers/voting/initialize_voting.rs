use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, NewBlockCipher};
use aes::{Aes256, Block, BLOCK_SIZE};
use num::Zero;
use rand::{seq::SliceRandom, thread_rng};

use uuid_b64::UuidB64 as Uuid;

use crate::auth::ClientToken;
use crate::db::DbConnection;
use crate::errors::{ResourceAction, ServiceError};
use crate::models::{Election, ElectionStatus, Registration};
use crate::protocol::generator_prime_pair;
use crate::utils::ConvertBigInt;

pub async fn initialize_voting(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_create_election()?;
  token.validate_user_id(&conn)?;

  // Make sure the election exists
  let mut election = Election::find_resource(&*path, &conn)?;

  // Only the election creator can open the election for voting
  let current_user_id = token.get_user_id();
  if election.created_by != current_user_id {
    return Err(ServiceError::ElectionNotOwnedByUser {
      current_user_id,
      owner_id: election.created_by,
      action: ResourceAction::InitVoting,
    });
  }

  // Make sure the election is in the correct
  if !(election.status == ElectionStatus::Registration || election.status == ElectionStatus::InitFailed) {
    return Err(ServiceError::WrongStatusFor {
      election_id: election.id,
      action: ResourceAction::InitVoting,
      status: election.status,
    });
  }

  // Election MUST have at least 2 users registered
  let registrations: Vec<Registration> = election.get_registrations(&conn)?;
  if registrations.len() < 2 {
    return Err(ServiceError::NotEnoughRegistered {
      election_id: election.id,
      num_registered: registrations.len(),
    });
  }

  // Mark the election as being initialized
  election.status = ElectionStatus::InitFailed;
  election = election.update(&conn)?;

  // We use a single prime that can serve the largest voting vector
  let questions_candidates = election.get_questions_candidates(&conn)?;
  let max_num_candidates = questions_candidates.iter().map(|(_, c)| c.len()).max().unwrap_or(2);
  let voting_vector_max_bits = registrations.len() * max_num_candidates;

  // Since we may call this method multiple times, only generate if we haven't done so before
  if election.generator.is_zero() || election.prime.is_zero() {
    let (generator, prime) = generator_prime_pair(2 * voting_vector_max_bits + 1);
    election.generator = generator.to_bigdecimal();
    election.prime = prime.to_bigdecimal();
    election = election.update(&conn)?;
  }

  // Encrypt the positions 0 to N using AES
  let key = GenericArray::from_slice(election.encryption_key.as_slice());
  let cipher = Aes256::new(&key);
  let mut encrypted_positions: Vec<Block> = (0u128..(registrations.len() as u128))
    .into_iter()
    .map(|i| Block::from(i.to_be_bytes()))
    .collect();
  cipher.encrypt_blocks(&mut encrypted_positions);

  // Then shuffle the list
  encrypted_positions.shuffle(&mut thread_rng());

  // TODO: Register the election with the first collector
  // TODO: Register the election with the second collector

  Ok(HttpResponse::Ok().finish())
}
