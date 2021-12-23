use curv_kzen::arithmetic::Converter;
use curv_kzen::BigInt;
use sha2::{Digest, Sha256};
use std::hash::Hasher;

/// Custom Rust hasher type that uses SHA-256 internally to compute the hash.
///  This ensures consistant behavior across Rust implementations.
pub struct SHAHasher {
  state: Sha256,
}

pub type ShaIntType = u32;
pub type Sha256Output = [ShaIntType; SHA_INT_ENTRIES];

const SHA_INT_ENTRIES: usize = (256 / ShaIntType::BITS) as usize;
const SHA_INT_BYTES: usize = (ShaIntType::BITS / 8) as usize;

impl SHAHasher {
  pub fn new() -> Self {
    Self { state: Sha256::new() }
  }

  /// Get the current hasher output as a big integer
  pub fn get_sha_hash(&self) -> BigInt {
    let current_hash = self.state.clone().finalize();
    BigInt::from_bytes(&current_hash[..])
  }

  /// Get the current hasher output as an array of u32
  #[allow(unused)]
  pub fn get_sha_ints(&self) -> Sha256Output {
    let current_hash = self.state.clone().finalize();

    // Convert into a byte array
    let mut int_array: Sha256Output = Default::default();
    for i in 0..SHA_INT_ENTRIES {
      let mut int_data: [u8; SHA_INT_BYTES] = Default::default();
      int_data.copy_from_slice(&current_hash[(i * SHA_INT_BYTES)..(i + 1) * SHA_INT_BYTES]);
      int_array[i] = ShaIntType::from_le_bytes(int_data);
    }

    int_array
  }
}

impl Hasher for SHAHasher {
  fn write(&mut self, bytes: &[u8]) {
    self.state.update(bytes);
  }

  fn finish(&self) -> u64 {
    // Clone the state to get the current hash value
    let current_hash = self.state.clone().finalize();

    // Convert into a byte array
    let mut u64_data: [u8; 8] = Default::default();
    u64_data.copy_from_slice(&current_hash[0..8]);
    u64::from_le_bytes(u64_data)
  }

  #[inline]
  fn write_u8(&mut self, i: u8) {
    self.write(&[i])
  }

  #[inline]
  fn write_u16(&mut self, i: u16) {
    self.write(&i.to_le_bytes())
  }

  #[inline]
  fn write_u32(&mut self, i: u32) {
    self.write(&i.to_le_bytes())
  }

  #[inline]
  fn write_u64(&mut self, i: u64) {
    self.write(&i.to_le_bytes())
  }

  #[inline]
  fn write_u128(&mut self, i: u128) {
    self.write(&i.to_le_bytes())
  }

  #[inline]
  fn write_usize(&mut self, i: usize) {
    self.write(&i.to_le_bytes())
  }

  #[inline]
  fn write_i8(&mut self, i: i8) {
    self.write_u8(i as u8)
  }

  #[inline]
  fn write_i16(&mut self, i: i16) {
    self.write_u16(i as u16)
  }

  #[inline]
  fn write_i32(&mut self, i: i32) {
    self.write_u32(i as u32)
  }

  #[inline]
  fn write_i64(&mut self, i: i64) {
    self.write_u64(i as u64)
  }

  #[inline]
  fn write_i128(&mut self, i: i128) {
    self.write_u128(i as u128)
  }

  #[inline]
  fn write_isize(&mut self, i: isize) {
    self.write_usize(i as usize)
  }
}
