//
// Environment configuration functions
//
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_JWT_SECRET: &str = "JWT_SECRET_VALUE_LOL";

/// Electronic voting Rest API server
#[derive(StructOpt)]
pub struct Opt {
  /// Host to run the server
  #[structopt(short, long, env, default_value = DEFAULT_HOST)]
  host: String,

  /// Port to use for the server
  #[structopt(short, long, env, default_value = "3000")]
  port: u16,

  /// Enable HTTPS (SSL) for the server
  #[structopt(long, env, takes_value(false), requires("key-file"), requires("cert-file"))]
  use_https: bool,

  /// Path for the SSL private key file
  #[structopt(long, env, parse(from_os_str))]
  key_file: Option<PathBuf>,

  /// Path for the SSL certificate chail file
  #[structopt(long, env, parse(from_os_str))]
  cert_file: Option<PathBuf>,

  /// Database connection URL
  #[structopt(long, env, hide_env_values = true)]
  database_url: String,

  /// JSON Web Token Secret
  #[structopt(short = "s", long, env, hide_env_values = true, default_value = DEFAULT_JWT_SECRET, hide_default_value(true))]
  jwt_secret: String,

  /// Secret key to verify Google reCAPTCHA
  #[structopt(short = "r", long, env, hide_env_values = true)]
  recaptcha_secret_key: String,

  /// Base URL that can be used to access collector 1
  #[structopt(long, env)]
  c1_url: String,

  /// Base URL that can be used to access collector 2
  #[structopt(long, env)]
  c2_url: String,
}

impl Opt {
  /// Update the environment variables with the command-line options
  pub fn update_environment(&self) {
    env::set_var("HOST", &self.host);
    env::set_var("PORT", &self.port.to_string());

    if self.use_https {
      env::set_var("USE_HTTPS", "true");
    }
    if let Some(ref key_file) = self.key_file {
      env::set_var("KEY_FILE", key_file);
    }
    if let Some(ref cert_file) = self.cert_file {
      env::set_var("CERT_FILE", cert_file);
    }

    env::set_var("DATABASE_URL", &self.database_url);
    env::set_var("JWT_SECRET", &self.jwt_secret);
    env::set_var("RECAPTCHA_SECRET_KEY", &self.recaptcha_secret_key);
    env::set_var("C1_URL", &self.c1_url);
    env::set_var("C2_URL", &self.c2_url);
  }
}

///
/// Load the .env files into the current environment
///
pub fn load_environment_from_env_files() {
  dotenv().ok(); /* .env file */
  if cfg!(debug_assertions) {
    dotenv::from_filename(".env.development").ok();
  } else {
    dotenv::from_filename(".env.production").ok();
  }
}

//
// Basic Server Variables
//
pub fn get_host() -> String {
  env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

pub fn get_port() -> u16 {
  env::var("PORT")
    .map(|port| port.parse().unwrap_or(DEFAULT_PORT))
    .unwrap_or(DEFAULT_PORT)
}

//
// HTTPS and SSL/TLS Encryption
//
pub fn use_https() -> bool {
  env::var("USE_HTTPS")
    .map(|https| https.parse().unwrap_or(false))
    .unwrap_or(false)
}

pub fn get_key_file() -> Option<String> {
  env::var("KEY_FILE").ok()
}

pub fn get_cert_file() -> Option<String> {
  env::var("CERT_FILE").ok()
}

//
// Database
//
pub fn get_database_url() -> Option<String> {
  env::var("DATABASE_URL").ok()
}

//
// Authentication System Secrets
//
pub fn get_jwt_secret() -> String {
  env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_JWT_SECRET.to_string())
}

pub fn get_recaptcha_secret_key() -> Option<String> {
  return env::var("RECAPTCHA_SECRET_KEY").ok();
}

//
// Collectors
//
pub fn get_c1_url() -> Option<String> {
  return env::var("C1_URL").ok();
}

pub fn get_c2_url() -> Option<String> {
  return env::var("C2_URL").ok();
}
