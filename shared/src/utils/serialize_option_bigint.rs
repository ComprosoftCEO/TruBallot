use curv_kzen::arithmetic::traits::Converter;
use curv_kzen::BigInt;
use serde::{de, ser};
use std::fmt;

pub fn serialize<S: ser::Serializer>(x: &Option<BigInt>, serializer: S) -> Result<S::Ok, S::Error> {
  match x {
    Some(x) => serializer.serialize_str(&x.to_str_radix(10)),
    None => serializer.serialize_none(),
  }
}

pub fn deserialize<'de, D: de::Deserializer<'de>>(deserializer: D) -> Result<Option<BigInt>, D::Error> {
  struct OptionBigIntVisitor;

  impl<'de> de::Visitor<'de> for OptionBigIntVisitor {
    type Value = Option<BigInt>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("option of bigint")
    }

    #[inline]
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
      Ok(None)
    }

    #[inline]
    fn visit_none<E: de::Error>(self) -> Result<Option<BigInt>, E> {
      Ok(None)
    }

    #[inline]
    fn visit_some<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
      kzen_paillier::serialize::bigint::deserialize(deserializer).map(Some)
    }
  }

  deserializer.deserialize_option(OptionBigIntVisitor)
}
