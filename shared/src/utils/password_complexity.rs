use zxcvbn::{zxcvbn, Entropy, ZxcvbnError};

/// Response from the password complexity check
#[derive(Debug, Clone)]
pub enum PasswordComplexityResponse {
  PasswordOkay,
  NotEnoughEntropy(Entropy),
  ZxcvbnError(ZxcvbnError),
}

// 0 - Too guessable: risky password. (guesses < 10^3)
// 1 - Very guessable: protection from throttled online attacks. (guesses < 10^6)
// 2 - Somewhat guessable: protection from unthrottled online attacks. (guesses < 10^8)
// 3 - Safely unguessable: moderate protection from offline slow-hash scenario. (guesses < 10^10)
// 4 - Very unguessable: strong protection from offline slow-hash scenario. (guesses >= 10^10)
const MINIMUM_PASSWORD_COMPLEXITY: u8 = 3;

const SITE_SPECIFIC_PASSWORD_DICTIONARY: &[&str] = &["evoting", "voting", "voter"];

/// Use the zxcvbn library to verify the password complexity
///
/// # Returns
/// Nested results for error handling
pub fn validate_password_complexity(password: &str, name: &str, email: &str) -> PasswordComplexityResponse {
  let mut user_inputs: Vec<&str> = vec![name, email];
  user_inputs.extend_from_slice(SITE_SPECIFIC_PASSWORD_DICTIONARY);

  match zxcvbn(password, user_inputs.as_slice()) {
    Ok(entropy) if entropy.score() >= MINIMUM_PASSWORD_COMPLEXITY => PasswordComplexityResponse::PasswordOkay,
    Ok(entropy) => entropy.into(),
    Err(e) => e.into(),
  }
}

impl From<Entropy> for PasswordComplexityResponse {
  fn from(entropy: Entropy) -> Self {
    Self::NotEnoughEntropy(entropy)
  }
}

impl From<ZxcvbnError> for PasswordComplexityResponse {
  fn from(error: ZxcvbnError) -> Self {
    Self::ZxcvbnError(error)
  }
}
