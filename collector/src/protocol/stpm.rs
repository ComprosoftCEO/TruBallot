use curv_kzen::arithmetic::{Modulo, Samplable};
use kzen_paillier::*;

/// First step of the STPM:
///   C1 computes e_x1 = E(x1) and sends it to C2
///
/// x1 = First number being multiplied
/// n  = Public key for Paillier cryptosystem
///
/// Returns e_x1
pub fn step_1(x1: &BigInt, n: &BigInt) -> BigInt {
  // Compute E(x1)
  let ek = EncryptionKey::from(n);
  let e_x1: RawCiphertext = Paillier::encrypt(&ek, x1.into());
  e_x1.0.into_owned()
}

/// Second step of the STPM:
///   C2 picks random r2 with 0 <= r2 < n
///   C2 computes e_x2 = ((e_x1)^x2) * (E(r2)^(-1)) (mod n^2) and sends it to C1
///
/// e_x1 = E(x1)
/// x2   = Second number being multiplied
/// n    = Public key for paillier cryptosystem
///
/// normalize = If true, then finds r1 + r2 = x1 * x2 with NO modulus
///   This means that r1 (or r2) may be negative
///
/// Returns (r2, e_x2)
pub fn step_2(e_x1: &BigInt, x2: &BigInt, n: &BigInt, normalize: bool) -> (BigInt, BigInt) {
  // Pick random r2 with 0 <= r2 < n
  let mut r2 = BigInt::sample_below(n);

  // Compute E(r2)
  let ek = EncryptionKey::from(n);
  let e_r2: RawCiphertext = Paillier::encrypt(&ek, r2.clone().into());

  // Compute ((e_x1)^x2) * (E(r2)^(-1)) (mod n^2)
  //  Every encrypted r value should be invertible, so we should never panic when finding inverse
  let e_x2 = BigInt::mod_mul(
    &BigInt::mod_pow(e_x1, &x2, &ek.nn),
    &BigInt::mod_inv(&e_r2.0, &ek.nn).expect("Error: No Inverse"),
    &ek.nn,
  );

  // If the "normalize" flag is set, then r1 + r2 = x1 * x2 with NO modulus
  //
  // So, if 2*r2 > n, then r2 = r2 - n
  //  -This will be a negative number
  if normalize && (&(2 * &r2) >= &n) {
    r2 = &r2 - n;
  }

  (r2, e_x2)
}

/// Last step of the STPM:
///   C1 decrypts D(e_x2), resulting in r1
///
/// The two parties now have values: r1 + r2 (mod n) = x1 * x2 (mod n^2)
/// If the "normalize" flag was set to true, then r1 + r2 = x1 * x2 with NO modulus
///
/// e_x2 = (E(x1)^x2) * (E(r2)^(-1)) (mod n^2)  -- From previous step
/// p, q = Private key for Paillier cryptosystem
///
/// normalize = If true, then finds r1 + r2 = x1 * x2 with NO modulus
///   This means that r1 (or r2) may be negative
///
/// Returns r1
pub fn step_3(e_x2: &BigInt, p: &BigInt, q: &BigInt, normalize: bool) -> BigInt {
  let e_x2 = RawCiphertext::from(e_x2);

  // Compute r1 = D(e_x2)
  let dk = DecryptionKey {
    p: p.clone(),
    q: q.clone(),
  };
  let r1: RawPlaintext = Paillier::decrypt(&dk, e_x2);

  // If the "normalize" flag is set, then r1 + r2 = x1 * x2 with NO modulus
  //
  // So, if 2*r1 > n, then r1 = r1 - n
  //  -This will be a negative number
  let n = p * q;
  let mut r1 = r1.0.into_owned();
  if normalize && (&(2 * &r1) >= &n) {
    r1 = &r1 - &n;
  }

  r1
}
