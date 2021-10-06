//
// Environment configuration functions
//
use crate::Collector;
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_C1_PORT: u16 = 3001;
const DEFAULT_C2_PORT: u16 = 3002;
const DEFAULT_JWT_SECRET: &str = "JWT_SECRET_VALUE_LOL";

/// Electronic voting collector daemon
#[derive(StructOpt)]
pub struct Opt {
  /// Index of the collector ("1" or "2")
  collector: Collector,

  /// Host to run the collector [Default: "127.0.0.1"]
  #[structopt(short, long)]
  host: Option<String>,

  /// Port to use for the collector [Default: C1 = 3001, C2 = 3002]
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

  /// Base URL that can be used to access collector 1
  #[structopt(long, env)]
  c1_url: String,

  /// Base URL that can be used to access collector 2
  #[structopt(long, env)]
  c2_url: String,
}

impl Opt {
  pub fn get_collector(&self) -> Collector {
    self.collector
  }

  /// Update the environment variables with the command-line options
  pub fn update_environment(&self) {
    env::set_var("COLLECTOR", self.collector.to_number().to_string());

    let c = self.collector;
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
pub fn get_collector() -> Collector {
  Collector::from_str(&env::var("COLLECTOR").unwrap_or_else(|_| "1".into())).unwrap_or_else(|_| Collector::One)
}

pub fn get_host() -> String {
  let c = get_collector();
  env::var(c.env_prefix("PORT")).unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

pub fn get_port() -> u16 {
  match get_collector() {
    Collector::One => env::var("C1_PORT")
      .map(|port| port.parse().unwrap_or(DEFAULT_C1_PORT))
      .unwrap_or(DEFAULT_C1_PORT),

    Collector::Two => env::var("C2_PORT")
      .map(|port| port.parse().unwrap_or(DEFAULT_C2_PORT))
      .unwrap_or(DEFAULT_C2_PORT),
  }
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
// Collectors
//
pub fn get_c1_url() -> Option<String> {
  return env::var("C1_URL").ok();
}

pub fn get_c2_url() -> Option<String> {
  return env::var("C2_URL").ok();
}
