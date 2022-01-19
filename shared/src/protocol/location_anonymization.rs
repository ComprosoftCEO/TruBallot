use curv_kzen::arithmetic::{Modulo, Samplable};
use kzen_paillier::*;

/// First step of the location anonymization scheme
///   C1 computes e_x1 = E(x1) and sends it to C2
///
/// location = Secret location (0 <= location <= n - 1)
/// n        = Public key for Paillier cryptosystem
///
/// Returns e_x1
pub fn step_1(location: &BigInt, n: &BigInt) -> BigInt {
  // Compute E(x1)
  let ek = EncryptionKey::from(n);
  let e_x1: RawCiphertext = Paillier::encrypt(&ek, location.into());
  e_x1.0.into_owned()
}

/// ith step of the location anonymization scheme: (Collectors C2 to C{n-1})
///   Ci picks random ri with 0 <= ri < n
///   Ci computes e_xi = (e_x1 * ... * e_x{i-1}) * (E(ri)^(-1)) (mod n^2) and sends it to C{i+1}
///
/// e_xi_1 = (e_x1 * ... * e_x{i-1})
/// n      = Public key for paillier cryptosystem
///
/// Returns (ri, e_xi)
pub fn step_ith(e_xi_1: &BigInt, n: &BigInt) -> (BigInt, BigInt) {
  // Pick random ri with 0 <= ri < n
  let ri = BigInt::sample_below(n);

  // Compute E(r2)
  let ek = EncryptionKey::from(n);
  let e_ri: RawCiphertext = Paillier::encrypt(&ek, ri.clone().into());

  // Compute (e_xi_1) * (E(ri)^(-1)) (mod n^2)
  //  Every encrypted r value should be invertible, so we should never panic when finding inverse
  let e_xi = BigInt::mod_mul(
    &e_xi_1,
    &BigInt::mod_inv(&e_ri.0, &ek.nn).expect("Error: No Inverse"),
    &ek.nn,
  );

  (ri, e_xi)
}

/// Last step of the location anonymization scheme:
///   C1 decrypts D(e_x{n-1}), resulting in r1
///
/// The parties now have values: r1 + r2 + ... + rn (mod n) = x1 (mod n^2)
///
/// e_xn_1 = (e_x1 * ... * e_x{n-1})
/// p, q   = Private key for Paillier cryptosystem
///
/// Returns r1
pub fn step_last(e_xn_1: &BigInt, p: &BigInt, q: &BigInt) -> BigInt {
  let e_xn_1 = RawCiphertext::from(e_xn_1);

  // Compute r1 = D(e_x2)
  let dk = DecryptionKey {
    p: p.clone(),
    q: q.clone(),
  };
  let r1: RawPlaintext = Paillier::decrypt(&dk, e_xn_1);

  r1.0.into_owned()
}
