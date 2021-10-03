use zxcvbn::zxcvbn;

use crate::errors::ServiceError;

// 0 - Too guessable: risky password. (guesses < 10^3)
// 1 - Very guessable: protection from throttled online attacks. (guesses < 10^6)
// 2 - Somewhat guessable: protection from unthrottled online attacks. (guesses < 10^8)
// 3 - Safely unguessable: moderate protection from offline slow-hash scenario. (guesses < 10^10)
// 4 - Very unguessable: strong protection from offline slow-hash scenario. (guesses >= 10^10)
const MINIMUM_PASSWORD_COMPLEXITY: u8 = 3;

const SITE_SPECIFIC_PASSWORD_DICTIONARY: &[&str] = &["evoting", "voting", "voter"];

pub fn validate_password_complexity(password: &str, name: &str, email: &str) -> Result<(), ServiceError> {
  let mut user_inputs: Vec<&str> = vec![name, email];
  user_inputs.extend_from_slice(SITE_SPECIFIC_PASSWORD_DICTIONARY);

  let entropy = zxcvbn(password, user_inputs.as_slice())?;

  if entropy.score() < MINIMUM_PASSWORD_COMPLEXITY {
    Err(entropy.into())
  } else {
    Ok(())
  }
}
