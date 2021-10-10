use bigdecimal::BigDecimal;
use curv_kzen::BigInt;
use kzen_paillier::DecryptionKey;
use rand::{thread_rng, Rng};
use serde::Serialize;
use uuid_b64::UuidB64 as Uuid;

use crate::schema::elections;
use crate::utils::ConvertBigInt;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[changeset_options(treat_none_as_null = "true")]
#[serde(rename_all = "camelCase")]
pub struct Election {
  pub id: Uuid,
  pub generator: BigDecimal,
  pub prime: BigDecimal,
  pub paillier_p: BigDecimal,
  pub paillier_q: BigDecimal,
  pub encryption_key: Vec<u8>,
}

impl Election {
  model_base!();

  has_many!(Registration);

  pub fn new(id: Uuid, generator: &BigInt, prime: &BigInt, paillier: &DecryptionKey) -> Self {
    // Convert from BigInt to BigDecimal
    let generator = generator.to_bigdecimal();
    let prime = prime.to_bigdecimal();

    let paillier_p = paillier.p.to_bigdecimal();
    let paillier_q = paillier.q.to_bigdecimal();

    // Generate a random AES encryption key
    let encryption_key = thread_rng().gen::<[u8; 32]>().to_vec();

    Self {
      id,
      generator,
      prime,
      paillier_p,
      paillier_q,
      encryption_key,
    }
  }
}
