use uuid_b64::UuidB64 as Uuid;

use super::is_offensive_string;

//
// Censor the base-64 UUID V4 text so that it contains no offensive or vulgar words
//   The probability of such words appearing is very small, but we still want to be extra safe
//   From experimental tests, only 1.6% to 1.7% of generated UUIDs even remotely fail the censorship check
//
pub fn new_safe_uuid_v4() -> Uuid {
  loop {
    let uuid = Uuid::new();
    if is_offensive_string(&uuid.to_istring()) {
      continue;
    }

    return uuid;
  }
}
