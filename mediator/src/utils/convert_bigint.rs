use bigdecimal::BigDecimal;
use curv_kzen::arithmetic::{BigInt, Converter, NumberTests};
use num::bigint::{BigInt as NumBigInt, Sign};

///
/// Trait used to simplify converting between BigInt and BigDecimal
///
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
    BigDecimal::new(NumBigInt::from_bytes_be(get_sign(self), &self.to_bytes()), 0)
  }

  fn from_bigint(bigint: BigInt) -> Self {
    bigint
  }

  fn from_bigdecimal(bigdecimal: BigDecimal) -> Self {
    let (bigint, _exponent) = bigdecimal.as_bigint_and_exponent();
    let (sign, bytes) = bigint.to_bytes_be();

    // Parse from bytes and handle the negative sign
    let result = BigInt::from_bytes(&bytes);
    if sign == Sign::Minus {
      -result
    } else {
      result
    }
  }
}

impl ConvertBigInt for BigDecimal {
  fn to_bigint(&self) -> BigInt {
    let (bigint, _exponent) = self.as_bigint_and_exponent();
    let (sign, bytes) = bigint.to_bytes_be();

    // Parse from bytes and handle the negative sign
    let result = BigInt::from_bytes(&bytes);
    if sign == Sign::Minus {
      -result
    } else {
      result
    }
  }

  fn to_bigdecimal(&self) -> BigDecimal {
    self.clone()
  }

  fn from_bigint(bigint: BigInt) -> Self {
    BigDecimal::new(NumBigInt::from_bytes_be(get_sign(&bigint), &bigint.to_bytes()), 0)
  }

  fn from_bigdecimal(bigdecimal: BigDecimal) -> Self {
    bigdecimal
  }
}

///
/// Figure out the sign of the input BigInt
///
fn get_sign(input: &BigInt) -> Sign {
  if BigInt::is_zero(input) {
    return Sign::NoSign;
  }

  if BigInt::is_negative(input) {
    Sign::Minus
  } else {
    Sign::Plus
  }
}
