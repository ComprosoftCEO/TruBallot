//
// Environment configuration functions
//
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_JWT_SECRET: &str = "JWT_SECRET_VALUE_LOL";

pub fn get_host() -> String {
  std::env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

pub fn get_port() -> u16 {
  std::env::var("PORT")
    .map(|port| port.parse().unwrap_or(DEFAULT_PORT))
    .unwrap_or(DEFAULT_PORT)
}

//
// HTTPS and SSL/TLS Encryption
//
pub fn use_https() -> bool {
  std::env::var("USE_HTTPS")
    .map(|https| https.parse().unwrap_or(false))
    .unwrap_or(false)
}

pub fn get_key_file() -> Option<String> {
  std::env::var("KEY_FILE").ok()
}

pub fn get_cert_file() -> Option<String> {
  std::env::var("CERT_FILE").ok()
}

//
// Database
//
pub fn get_database_url() -> Option<String> {
  std::env::var("DATABASE_URL").ok()
}

//
// Authentication System Secrets
//
pub fn get_jwt_secret() -> String {
  std::env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_JWT_SECRET.to_string())
}

pub fn get_recaptcha_secret_key() -> Option<String> {
  return std::env::var("RECAPTCHA_SECRET_KEY").ok();
}
