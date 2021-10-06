//
// Environment configuration functions
//
use crate::Collector;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_C1_PORT: u16 = 3001;
const DEFAULT_C2_PORT: u16 = 3002;
const DEFAULT_JWT_SECRET: &str = "JWT_SECRET_VALUE_LOL";

pub fn get_host(c: Collector) -> String {
  match c {
    Collector::One => std::env::var("C1_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string()),
    Collector::Two => std::env::var("C2_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string()),
  }
}

pub fn get_port(c: Collector) -> u16 {
  match c {
    Collector::One => std::env::var("C1_PORT")
      .map(|port| port.parse().unwrap_or(DEFAULT_C1_PORT))
      .unwrap_or(DEFAULT_C1_PORT),

    Collector::Two => std::env::var("C2_PORT")
      .map(|port| port.parse().unwrap_or(DEFAULT_C2_PORT))
      .unwrap_or(DEFAULT_C2_PORT),
  }
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
pub fn get_database_url(c: Collector) -> Option<String> {
  match c {
    Collector::One => std::env::var("C1_DATABASE_URL").ok(),
    Collector::Two => std::env::var("C2_DATABASE_URL").ok(),
  }
}

//
// Authentication System Secrets
//
pub fn get_jwt_secret() -> String {
  std::env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_JWT_SECRET.to_string())
}
