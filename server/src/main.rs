use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use simple_logger::SimpleLogger;

use evoting_server::config;
use evoting_server::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load our ".env" configuration files
  dotenv().ok();
  if cfg!(debug_assertions) {
    dotenv::from_filename(".env.development").ok();
  } else {
    dotenv::from_filename(".env.production").ok();
  }

  // Configure the logger system
  SimpleLogger::new().init().unwrap();
  if cfg!(debug_assertions) {
    log::set_max_level(LevelFilter::Debug);
  } else {
    log::set_max_level(LevelFilter::Info);
  }

  let mut server = HttpServer::new(|| {
    App::new()
      .data(db::establish_new_connection_pool())
      // Enable logger
      .wrap(middleware::Logger::default())
      // Limit amount of data the server will accept
      .data(web::JsonConfig::default().limit(4096))
      // Load all routes
      .route("/", web::get().to(|| HttpResponse::Forbidden().finish()))
  });

  // Possibly enable SSL
  let ip_port = format!("{}:{}", config::get_host(), config::get_port());
  server = if config::use_https() {
    server.bind_openssl(ip_port, get_ssl_configuration())?
  } else {
    server.bind(ip_port)?
  };

  // Run and listen for connections
  server.run().await
}

/// Load and configure SSL if required
///
/// Panicks if SSL cannot be configured
fn get_ssl_configuration() -> SslAcceptorBuilder {
  let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

  // Load private key
  acceptor
    .set_private_key_file(
      config::get_key_file().expect("Missing Private Key File"),
      SslFiletype::PEM,
    )
    .unwrap();

  // Load certificate chain
  acceptor
    .set_certificate_chain_file(config::get_cert_file().expect("Missing Certificate File"))
    .unwrap();

  // Validate our SSL connection
  acceptor.check_private_key().unwrap();

  acceptor
}
