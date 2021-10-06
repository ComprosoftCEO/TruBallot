use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use simple_logger::SimpleLogger;
use std::error::Error;
use structopt::StructOpt;

use evoting_collector::auth;
use evoting_collector::config;
use evoting_collector::db;
use evoting_collector::errors::ServiceError;
use evoting_collector::handlers;
use evoting_collector::Collector;

/// Electronic voting collector daemon
#[derive(Clone, StructOpt)]
struct Opt {
  /// Index of the collector ("1" or "2")
  collector: Collector,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  // Load our ".env" configuration files
  dotenv().ok();
  if cfg!(debug_assertions) {
    dotenv::from_filename(".env.development").ok();
  } else {
    dotenv::from_filename(".env.production").ok();
  }

  // Parse command-line arguments
  let opt = Opt::from_args();

  // Configure the logger system
  SimpleLogger::new().init()?;
  if cfg!(debug_assertions) {
    log::set_max_level(LevelFilter::Debug);
  } else {
    log::set_max_level(LevelFilter::Info);
  }

  // Server URL
  let ip_port = format!(
    "{}:{}",
    config::get_host(opt.collector),
    config::get_port(opt.collector)
  );

  // Database connection
  let connection_pool = db::establish_new_connection_pool(opt.collector)?;

  let mut server = HttpServer::new(move || {
    App::new()
      // Connect to database
      .data(connection_pool.clone())
      // Encryption secret for JSON Web Token
      .data(auth::JWTSecret::new(config::get_jwt_secret()))
      // Store the collector
      .data(opt.collector)
      // Enable logger
      .wrap(middleware::Logger::default())
      // Limit amount of data the server will accept
      .data(web::JsonConfig::default().limit(4096))
      // Load all routes
      .service(web::scope("/api/v1").service(web::scope("/auth").route("", web::get().to(handlers::auth::get_me))))
      .default_service(web::route().to(|| HttpResponse::NotFound()))
  });

  // Possibly enable SSL
  server = if config::use_https() {
    server.bind_openssl(ip_port, get_ssl_configuration()?)?
  } else {
    server.bind(ip_port)?
  };

  // Run and listen for connections
  Ok(server.run().await?)
}

///
/// Load and configure SSL if required
///
fn get_ssl_configuration() -> anyhow::Result<SslAcceptorBuilder> {
  let private_key_file = config::get_key_file()
    .ok_or_else(|| ServiceError::SSLConfigurationError("KEY_FILE environment variable not set".into()))?;

  let cert_file = config::get_cert_file()
    .ok_or_else(|| ServiceError::SSLConfigurationError("CERT_FILE environment variable not set".into()))?;

  let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

  // Load private key and certificate chain, then validate our SSL configuration
  acceptor.set_private_key_file(private_key_file, SslFiletype::PEM)?;
  acceptor.set_certificate_chain_file(cert_file)?;
  acceptor.check_private_key()?;

  Ok(acceptor)
}
