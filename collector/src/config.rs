//
// Environment configuration functions
//
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use uuid_b64::UuidB64 as Uuid;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_C0_PORT: u16 = 4000;
const DEFAULT_JWT_SECRET: &str = "JWT_SECRET_VALUE_LOL";

/// Get the prefix to use when loading collector-specific environment variables
pub trait EnvPrefix {
  fn env_prefix(&self, env: &str) -> String;
}

impl EnvPrefix for u64 {
  fn env_prefix(&self, env: &str) -> String {
    format!("C{}_{}", self, env)
  }
}

/// Electronic voting collector daemon
#[derive(StructOpt)]
pub struct Opt {
  /// Index of the collector (1, 2, ...)
  collector: u64,

  /// Unique UUID for the collector
  #[structopt(long)]
  id: Option<Uuid>,

  /// Name of the collector as a human-readable string
  #[structopt(short, long)]
  name: Option<String>,

  /// Host to run the collector [Default: "127.0.0.1"]
  #[structopt(short, long)]
  host: Option<String>,

  /// Port to use for the collector [Default: C1 = 4001, C2 = 4002, ...]
  #[structopt(short, long)]
  port: Option<u16>,

  /// Enable HTTPS (SSL) for the server
  #[structopt(long)]
  use_https: bool,

  /// Path for the SSL private key file
  #[structopt(long, parse(from_os_str))]
  key_file: Option<PathBuf>,

  /// Path for the SSL certificate chail file
  #[structopt(long, parse(from_os_str))]
  cert_file: Option<PathBuf>,

  /// Database connection URL
  #[structopt(long)]
  database_url: Option<String>,

  /// JSON Web Token Secret
  #[structopt(short = "s", long, env, hide_env_values = true, default_value = DEFAULT_JWT_SECRET, hide_default_value(true))]
  jwt_secret: String,

  /// Base URL that can be used to access the collector mediators
  #[structopt(long, env)]
  mediator_url: String,
}

impl Opt {
  /// Update the environment variables with the command-line options
  pub fn update_environment(&self) {
    let c = self.collector;
    env::set_var("COLLECTOR", c.to_string());

    if let Some(ref id) = self.id {
      env::set_var(c.env_prefix("ID"), id.to_string());
    }
    if let Some(ref name) = self.name {
      env::set_var(c.env_prefix("NAME"), name);
    }

    if let Some(ref host) = self.host {
      env::set_var(c.env_prefix("HOST"), host);
    }
    if let Some(ref port) = self.port {
      env::set_var(c.env_prefix("PORT"), port.to_string());
    }

    if self.use_https {
      env::set_var(c.env_prefix("USE_HTTPS"), "true");
    }
    if let Some(ref key_file) = self.key_file {
      env::set_var(c.env_prefix("KEY_FILE"), key_file);
    }
    if let Some(ref cert_file) = self.cert_file {
      env::set_var(c.env_prefix("CERT_FILE"), cert_file);
    }

    if let Some(ref database_url) = self.database_url {
      env::set_var(c.env_prefix("DATABASE_URL"), database_url);
    }

    env::set_var("JWT_SECRET", &self.jwt_secret);
    env::set_var("MEDIATOR_URL", &self.mediator_url);
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
// Collector Variables
//
pub fn get_collector() -> u64 {
  // Note: collector CANNOT be smaller than 1 (0 is NOT allowed)
  let c = u64::from_str(&env::var("COLLECTOR").unwrap_or_else(|_| "1".into())).unwrap_or_else(|_| 1);
  std::cmp::max(c, 1)
}

pub fn get_id() -> Option<Uuid> {
  env::var(get_collector().env_prefix("ID"))
    .ok()
    .and_then(|uuid| uuid.parse().ok())
}

pub fn get_name() -> String {
  let c = get_collector();
  env::var(c.env_prefix("NAME")).unwrap_or_else(|_| format!("Collector {}", c))
}

//
// Basic Server Variables
//
pub fn get_host() -> String {
  let c = get_collector();
  env::var(c.env_prefix("HOST")).unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

pub fn get_port() -> u16 {
  let c = get_collector();
  let default_port: u16 = DEFAULT_C0_PORT + (c as u16);

  env::var(c.env_prefix("PORT"))
    .map(|port| port.parse().unwrap_or(default_port))
    .unwrap_or(default_port)
}

//
// HTTPS and SSL/TLS Encryption
//
pub fn use_https() -> bool {
  let c = get_collector();
  env::var(c.env_prefix("USE_HTTPS"))
    .map(|https| https.parse().unwrap_or(false))
    .unwrap_or(false)
}

pub fn get_key_file() -> Option<String> {
  let c = get_collector();
  env::var(c.env_prefix("KEY_FILE")).ok()
}

pub fn get_cert_file() -> Option<String> {
  let c = get_collector();
  env::var(c.env_prefix("CERT_FILE")).ok()
}

//
// Database
//
pub fn get_database_url() -> Option<String> {
  let c = get_collector();
  env::var(c.env_prefix("DATABASE_URL")).ok()
}

//
// Authentication System Secrets
//
pub fn get_jwt_secret() -> String {
  env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_JWT_SECRET.to_string())
}

//
// Mediator
//
pub fn get_mediator_url() -> Option<String> {
  return env::var("MEDIATOR_URL").ok();
}
