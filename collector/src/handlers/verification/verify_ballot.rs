use actix_web::client::Client;
use actix_web::{web, HttpResponse};
use curv_kzen::arithmetic::Modulo;
use curv_kzen::BigInt;
use kzen_paillier::*;
use uuid_b64::UuidB64 as Uuid;
use validator::Validate;

use super::websocket_messages::*;
use super::VerifyBallotData;
use crate::auth::{AnyToken, CollectorToken, JWTSecret, DEFAULT_PERMISSIONS};
use crate::db::DbConnection;
use crate::errors::{ServiceError, WebsocketError};
use crate::models::{Election, Question};
use crate::utils::ConvertBigInt;
use crate::views::verification::VerificationResult;
use crate::Collector;

pub async fn verify_ballot(
  token: AnyToken,
  path: web::Path<(Uuid, Uuid)>,
  data: web::Json<VerifyBallotData>,
  collector: web::Data<Collector>,
  conn: DbConnection,
  jwt_secret: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  token.test_can_view_elections()?;
  data.validate()?;

  let (election_id, question_id) = path.into_inner();
  let data = data.into_inner();

  // Make sure the election, question, and user registration exist
  let election = Election::find_resource(&election_id, &conn)?;
  let question = Question::find_resource(&question_id, &election_id, &conn)?;
  let num_registered = election.count_registrations(&conn)?;
  let registration = election
    .get_registration(&question_id, &data.user_id, &conn)?
    .ok_or_else(|| ServiceError::UserNotRegistered {
      user_id: data.user_id,
      election_id: election.id,
      question_id: Some(question_id),
    })?;

  // Various election parameters
  let generator = election.generator.to_bigint();
  let prime = election.prime.to_bigint();

  let s_i_c1 = registration.forward_verification_shares.to_bigint(); // S_i,C1
  let s_i_c1_prime = registration.reverse_verification_shares.to_bigint(); // S_i,C1'
  let stild_i_c1 = registration.forward_ballot_shares.to_bigint(); // S~i,C1
  let stild_i_c1_prime = registration.reverse_ballot_shares.to_bigint(); // S~i,C1'

  // Paillier keys for secure two-party multiplication (STPM)
  let (encryption_key, decryption_key) = Keypair {
    p: election.paillier_p.to_bigint(),
    q: election.paillier_q.to_bigint(),
  }
  .keys();

  // ============================================================
  // Open client websocket connection
  // ============================================================
  let url = format!(
    "{}://{}",
    if collector.opposite().is_secure() { "wss" } else { "ws" },
    collector.opposite().api_url(&format!(
      "/elections/{}/questions/{}/verification/ws/{}",
      election_id, question_id, data.user_id,
    ))?
  );

  let request = Client::builder()
    .disable_timeout()
    .finish()
    .ws(url)
    .bearer_auth(CollectorToken::new(DEFAULT_PERMISSIONS).encode(&jwt_secret.get_encoding_key())?);

  log::debug!("Connect to websocket");
  let mut stream = WebsocketError::connect(request)
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;

  // ============================================================
  // Send initialization
  // ============================================================
  log::debug!("Send initialization parameters");
  WebsocketError::send_json(
    &mut stream,
    &Initialize {
      forward_ballot: data.forward_ballot.clone(),
      reverse_ballot: data.reverse_ballot.clone(),
      g_s: data.g_s.clone(),
      g_s_prime: data.g_s_prime.clone(),
      g_s_s_prime: data.g_s_s_prime.clone(),
      n: encryption_key.n.clone(),
    },
  )
  .await
  .map_err(|e| ServiceError::VerificationError(e))?;

  // ============================================================
  // Sub-Protocol 1 - First Secure Two-Party Multiplication Request
  //
  // Computes r1 + r2' = S_i,C1 * S_i,C2'
  // ============================================================
  log::debug!("Sub-protocol 1: First STPM: r1 + r2' = S_i,C1 * S_i,C2'");

  // Step 1: Compute E(S_i,C1, e)
  let e_s_c1: RawCiphertext = Paillier::encrypt(&encryption_key, s_i_c1.clone().into());
  WebsocketError::send_json(
    &mut stream,
    &SP1_STMP1_Request {
      e_s_c1: e_s_c1.0.into_owned(),
    },
  )
  .await
  .map_err(|e| ServiceError::VerificationError(e))?;

  // Step 2: Server picks r2' and computes E(S_i,C1, e)^(S_i,C2')) * (E(r2', e)^(-1))
  let SP1_STMP1_Response { e_s_c1_e_r2_prime } = WebsocketError::read_json(&mut stream)
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;

  // Step 3: Compute r1 = D(E(S_i,C1, e)^(S_i,C2')) * (E(r2', e)^(-1)), d)
  let r1: RawPlaintext = Paillier::decrypt(&decryption_key, RawCiphertext::from(&e_s_c1_e_r2_prime));
  let r1 = r1.0.into_owned();
  log::debug!("r1 = {}", r1);

  // ============================================================
  // Sub-Protocol 1 - Second Secure Two-Party Multiplication Request
  //
  // Computes r1' + r2 = S_i,C1' * S_i,C2
  // ============================================================
  log::debug!("Sub-protocol 1: Second STPM: r1' + r2 = S_i,C1' * S_i,C2");

  // Step 1: Compute E(S_i,C1', e)
  let e_s_c1_prime: RawCiphertext = Paillier::encrypt(&encryption_key, s_i_c1_prime.clone().into());
  WebsocketError::send_json(
    &mut stream,
    &SP1_STMP2_Request {
      e_s_c1_prime: e_s_c1_prime.0.into_owned(),
    },
  )
  .await
  .map_err(|e| ServiceError::VerificationError(e))?;

  // Step 2: Server picks r2 and computes E(S_i,C1', e)^(S_i,C2)) * (E(r2, e)^(-1))
  let SP1_STMP2_Response { e_s_c1_prime_e_r2 } = WebsocketError::read_json(&mut stream)
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;

  // Step 3: Compute r1' = D(E(S_i,C1', e)^(S_i,C2)) * (E(r2, e)^(-1)), d)
  let r1_prime: RawPlaintext = Paillier::decrypt(&decryption_key, RawCiphertext::from(&e_s_c1_prime_e_r2));
  let r1_prime = r1_prime.0.into_owned();
  log::debug!("r1' = {}", r1_prime);

  // ============================================================
  // Sub-Protocol 1 - Compute combined products P1 and P2
  // ============================================================
  log::debug!("Sub-protocol 1: Computing combined products P1 and P2");

  // Step 1: Compute and send P1
  let p1 = BigInt::mod_mul(
    &BigInt::mod_mul(
      &BigInt::mod_pow(&data.g_s, &s_i_c1_prime, &prime),
      &BigInt::mod_pow(&data.g_s_prime, &s_i_c1, &prime),
      &prime,
    ),
    &BigInt::mod_mul(
      &BigInt::mod_pow(&generator, &(&s_i_c1 * &s_i_c1_prime), &prime),
      &BigInt::mod_pow(&generator, &(&r1 + &r1_prime), &prime),
      &prime,
    ),
    &prime,
  );
  log::debug!("P1 = {}", p1);

  WebsocketError::send_json(&mut stream, &SP1_Product1_Request { p1: p1.clone() })
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;

  // Step 2: Server sends back P2
  let SP1_Product2_Response { p2 } = WebsocketError::read_json(&mut stream)
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;
  log::debug!("P2 = {}", p2);

  // ============================================================
  // Sub-Protocol 1 - Final Verification
  // ============================================================
  log::debug!("Sub-protocol 1: Final verification");

  // Compute the combined product: g^(s_i * s_i') * P1 * P2
  let combined_product = BigInt::mod_mul(&data.g_s_s_prime, &BigInt::mod_mul(&p1, &p2, &prime), &prime);
  log::debug!("g^(s_i * s_i') * P1 * P2 = {}", combined_product);

  // Compute the expected product: g^(2^(L - 1)), where L is the number of bits in the voting vector
  let expected_product = BigInt::mod_pow(
    &generator,
    &(BigInt::from(1) << (num_registered * question.num_candidates - 1) as usize),
    &prime,
  );

  // Validate the client
  let sp1_c1_verified = combined_product == expected_product;
  log::debug!("g^(2^(L - 1)) = {}", expected_product);
  log::debug!("Client: ballot {}", if sp1_c1_verified { "valid" } else { "invalid" });

  // Validate the server
  let SP1_Result_Response {
    ballot_valid: sp1_c2_verified,
  } = WebsocketError::read_json(&mut stream)
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;
  log::debug!("Server: ballot {}", if sp1_c2_verified { "valid" } else { "invalid" });

  // Validate both client and server
  let sp1_verified = sp1_c1_verified && sp1_c2_verified;

  // ============================================================
  // Sub-Protocol 2 - Compute g values
  // ============================================================
  log::debug!("Sub-protocol 2 - Compute g values");

  // Step 1: Compute values g^(S~i,C1) and g^(S~i,C1')
  let g_stild_1 = BigInt::mod_pow(&generator, &stild_i_c1, &prime);
  let g_stild_1_prime = BigInt::mod_pow(&generator, &stild_i_c1_prime, &prime);
  log::debug!("g^(S~i,C1) = {}", g_stild_1);
  log::debug!("g^(S~i,C1') = {}", g_stild_1_prime);

  WebsocketError::send_json(
    &mut stream,
    &SP2_C1_Request {
      g_stild_1: g_stild_1.clone(),
      g_stild_1_prime: g_stild_1_prime.clone(),
    },
  )
  .await
  .map_err(|e| ServiceError::VerificationError(e))?;

  // Step 2: server computes values g^(S~i,C2) and g^(S~i,C2')
  let SP2_C2_Response {
    g_stild_2,
    g_stild_2_prime,
  } = WebsocketError::read_json(&mut stream)
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;

  log::debug!("g^(S~i,C2) = {}", g_stild_2);
  log::debug!("g^(S~i,C2') = {}", g_stild_2_prime);

  // ============================================================
  // Sub-Protocol 2 - Final Verification
  // ============================================================

  // Verify forward ballot on client
  let g_p_i = BigInt::mod_mul(&generator, &data.forward_ballot, &prime);
  let g_p_i_combined = BigInt::mod_mul(&data.g_s, &BigInt::mod_mul(&g_stild_1, &g_stild_2, &prime), &prime);
  let g_p_i_verified = &g_p_i == &g_p_i_combined;

  log::debug!("g^(p_i) = {}", g_p_i);
  log::debug!("g^(s_i) * g^(S~i,C1) * g^(S~i,C2) = {}", g_p_i_combined);
  log::debug!("Forward ballot: {}", if g_p_i_verified { "valid" } else { "invalid" });

  // Verify reverse ballot on client
  let g_p_i_prime = BigInt::mod_mul(&generator, &data.reverse_ballot, &prime);
  let g_p_i_prime_combined = BigInt::mod_mul(
    &data.g_s_prime,
    &BigInt::mod_mul(&g_stild_1_prime, &g_stild_2_prime, &prime),
    &prime,
  );
  let g_p_i_prime_verified = &g_p_i_prime == &g_p_i_prime_combined;

  log::debug!("g^(p_i') = {}", g_p_i_prime);
  log::debug!("g^(s_i') * g^(S~i,C1') * g^(S~i,C2') = {}", g_p_i_prime_combined);
  log::debug!(
    "Reverse ballot: {}",
    if g_p_i_prime_verified { "valid" } else { "invalid" }
  );

  // Combined verification
  let sp2_c1_verified = g_p_i_verified && g_p_i_prime_verified;

  // Verify on server
  let SP2_Result_Response {
    ballot_valid: sp2_c2_verified,
  } = WebsocketError::read_json(&mut stream)
    .await
    .map_err(|e| ServiceError::VerificationError(e))?;
  log::debug!("Server: ballot {}", if sp2_c2_verified { "valid" } else { "invalid" });

  // Validate both client and server
  let sp2_verified = sp2_c1_verified && sp2_c2_verified;

  // ============================================================
  // Return final verification results
  // ============================================================
  Ok(HttpResponse::Ok().json(VerificationResult {
    sub_protocol_1: sp1_verified,
    sub_protocol_2: sp2_verified,
  }))
}
