use http::Uri;
use serde::{de, ser};
use std::fmt;

pub fn serialize<S: ser::Serializer>(x: &Option<Uri>, serializer: S) -> Result<S::Ok, S::Error> {
  match x {
    Some(x) => http_serde::uri::serialize(x, serializer),
    None => serializer.serialize_none(),
  }
}

pub fn deserialize<'de, D: de::Deserializer<'de>>(deserializer: D) -> Result<Option<Uri>, D::Error> {
  struct OptionUriVisitor;

  impl<'de> de::Visitor<'de> for OptionUriVisitor {
    type Value = Option<Uri>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("option of uri")
    }

    #[inline]
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
      Ok(None)
    }

    #[inline]
    fn visit_none<E: de::Error>(self) -> Result<Option<Uri>, E> {
      Ok(None)
    }

    #[inline]
    fn visit_some<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
      http_serde::uri::deserialize(deserializer).map(Some)
    }
  }

  deserializer.deserialize_option(OptionUriVisitor)
}
