use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

use evoting_server::auth;
use evoting_server::config;
use evoting_server::db;
use evoting_server::errors::ServiceError;
use evoting_server::handlers;

/// Electronic voting Rest API server
#[derive(Clone, StructOpt)]
struct Opt {
  /// Host to run the server (Default: 127.0.0.1)
  #[structopt(short, long)]
  host: Option<String>,

  /// Port to use for the server (Default: 3000)
  #[structopt(short, long)]
  port: Option<u16>,
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

  // Host and port from command-line override environment variables
  let host = opt.host.clone().unwrap_or_else(|| config::get_host());
  let port = opt.port.unwrap_or_else(|| config::get_port());
  std::env::set_var("HOST", &host);
  std::env::set_var("PORT", port.to_string());

  // Configure the logger system
  SimpleLogger::new().init().unwrap();
  if cfg!(debug_assertions) {
    log::set_max_level(LevelFilter::Debug);
  } else {
    log::set_max_level(LevelFilter::Info);
  }

  // Database connection
  let connection_pool = db::establish_new_connection_pool()?;

  let mut server = HttpServer::new(move || {
    App::new()
      // Connect to database
      .data(connection_pool.clone())
      // Encryption secret for JSON Web Token
      .data(auth::JWTSecret::new(config::get_jwt_secret()))
      // Enable logger
      .wrap(middleware::Logger::default())
      // Limit amount of data the server will accept
      .data(web::JsonConfig::default().limit(4096))
      // Load all routes
      .service(
        web::scope("/api/v1").service(
          web::scope("/auth")
            .route("", web::get().to(handlers::auth::get_me))
            .route("/login", web::post().to(handlers::auth::login))
            .route("/refresh", web::post().to(handlers::auth::refresh)),
        ),
      )
      .default_service(web::route().to(|| HttpResponse::NotFound()))
  });

  // Possibly enable SSL
  let ip_port = format!("{}:{}", config::get_host(), config::get_port());
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
