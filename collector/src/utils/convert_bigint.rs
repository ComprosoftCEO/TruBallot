use bigdecimal::BigDecimal;
use curv_kzen::{arithmetic::Converter, BigInt};
use num::bigint::{BigInt as NumBigInt, Sign};

/// Trait used to simplify converting between BigInt and BigDecimal
pub trait ConvertBigInt {
  fn to_bigint(&self) -> BigInt;
  fn to_bigdecimal(&self) -> BigDecimal;

  fn from_bigint(bigint: BigInt) -> Self;
  fn from_bigdecimal(bigdecimal: BigDecimal) -> Self;
}

impl ConvertBigInt for BigInt {
  fn to_bigint(&self) -> BigInt {
    self.clone()
  }

  fn to_bigdecimal(&self) -> BigDecimal {
    BigDecimal::new(NumBigInt::from_bytes_be(Sign::Plus, &self.to_bytes()), 0)
  }

  fn from_bigint(bigint: BigInt) -> Self {
    bigint
  }

  fn from_bigdecimal(bigdecimal: BigDecimal) -> Self {
    BigInt::from_bytes(&bigdecimal.as_bigint_and_exponent().0.to_bytes_be().1)
  }
}

impl ConvertBigInt for BigDecimal {
  fn to_bigint(&self) -> BigInt {
    BigInt::from_bytes(&self.as_bigint_and_exponent().0.to_bytes_be().1)
  }

  fn to_bigdecimal(&self) -> BigDecimal {
    self.clone()
  }

  fn from_bigint(bigint: BigInt) -> Self {
    BigDecimal::new(NumBigInt::from_bytes_be(Sign::Plus, &bigint.to_bytes()), 0)
  }

  fn from_bigdecimal(bigdecimal: BigDecimal) -> Self {
    bigdecimal
  }
}
