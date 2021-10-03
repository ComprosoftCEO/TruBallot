use crate::config;
use crate::errors::ServiceError;

// Verify a reCAPTCHA token with Google reCAPTCHA
pub async fn verify_recaptcha(recaptcha: &str) -> Result<(), ServiceError> {
  let recaptcha_key = config::get_recaptcha_secret_key().ok_or(ServiceError::MissingRecaptchaSecret)?;
  recaptcha::verify(&recaptcha_key, &recaptcha, None)
    .await
    .map_err(|e| ServiceError::RecaptchaFailed(e))
}
