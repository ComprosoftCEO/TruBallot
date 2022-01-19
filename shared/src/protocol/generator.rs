use curv_kzen::arithmetic::{Modulo, One};
use curv_kzen::BigInt;
use kzen_paillier::keygen::PrimeSampable;

/// Returns a (generator, prime) pair for Z*p with at least n bits
///
/// We generate a safe prime p = 2*q + 1, where q is also prime.
///
/// To find the primitive root, we use Euler's totient function Φ(p) = p - 1.
/// Start by factorizing Φ(p). Since p-1 = 2*q, our factors are always 2 and q.
///
/// Then, for all potential g in 2 ..= Φ(p) and all factors f of Φ(p), we confirm:
///
///    g^(Φ(p)/f) != 1 (mod p)
///
/// If this is true for every factor of Φ(p), then g is indeed a primitive root.
///
/// Since p is prime, this is GUARANTEED to return a valid primitive root.
pub fn generator_prime_pair(num_bits: usize) -> (BigInt, BigInt) {
  // Pick a random large prime
  let p = BigInt::sample_safe_prime(num_bits);

  // To find primitive root, we consider the Euler's totient Φ(p)
  //   Since p is prime, Φ(p) = (p-1)
  let totient = &p - 1;

  // Since p is a safe prime, we know p = 2*q + 1
  //  So the prime factors of Φ(p) = p-1 = 2*q are always [2, q]
  let factors = vec![BigInt::from(2), &totient / 2];

  // There ALWAYS exists a primitive root somewhere 2 ..= Φ(p)
  let mut g = BigInt::from(1);
  'next_generator: while &g <= &totient {
    g = &g + 1;

    for factor in &factors {
      // Check if g^(Φ(p) / factor) != 1 (mod p)
      if BigInt::mod_pow(&g, &(&totient / factor), &p).is_one() {
        continue 'next_generator;
      }
    }

    // We found one!
    break;
  }

  (g, p)
}
