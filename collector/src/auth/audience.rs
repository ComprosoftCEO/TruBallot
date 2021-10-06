use std::collections::HashSet;
use std::iter::FromIterator;

/// Generic trait shared by all audience types
///
/// An audience specifies which routes a JWT can access
pub trait Audience {
  const TEXT: &'static str;
  const ACCEPTS: &'static [&'static str];

  /// Get the name to show for this audience type
  fn get_name() -> String {
    Self::TEXT.to_string()
  }

  /// Get the list of all audiences accepted by this type
  fn accepts() -> HashSet<String> {
    HashSet::from_iter(Self::ACCEPTS.iter().cloned().map(String::from))
  }
}

/// Route is available to all token types
pub struct All;

impl Audience for All {
  const TEXT: &'static str = "all";
  const ACCEPTS: &'static [&'static str] = &["all", "client-only", "server-only", "collector-only"];
}

/// Route can only be called by the frontend client
pub struct ClientOnly;

impl Audience for ClientOnly {
  const TEXT: &'static str = "client-only";
  const ACCEPTS: &'static [&'static str] = &["client-only"];
}

/// Route can only be called by the server
pub struct ServerOnly;

impl Audience for ServerOnly {
  const TEXT: &'static str = "server-only";
  const ACCEPTS: &'static [&'static str] = &["server-only"];
}

/// Route can only be called by a collector
pub struct CollectorOnly;

impl Audience for CollectorOnly {
  const TEXT: &'static str = "collector-only";
  const ACCEPTS: &'static [&'static str] = &["collector-only"];
}
