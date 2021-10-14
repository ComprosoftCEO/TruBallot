use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

use evoting_collector::auth;
use evoting_collector::config;
use evoting_collector::db;
use evoting_collector::errors::ServiceError;
use evoting_collector::handlers;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  // Parse ".env" configuration files and command-line arguments
  config::load_environment_from_env_files();

  let opt = config::Opt::from_args();
  let collector = opt.get_collector();
  opt.update_environment();

  // Configure the logger system
  SimpleLogger::new().init()?;
  if cfg!(debug_assertions) {
    log::set_max_level(LevelFilter::Debug);
  } else {
    log::set_max_level(LevelFilter::Info);
  }

  // Database connection pool and web server
  let connection_pool = db::establish_new_connection_pool()?;
  let mut server = HttpServer::new(move || {
    App::new()
      // Connect to database
      .data(connection_pool.clone())
      // Encryption secret for JSON Web Token
      .data(auth::JWTSecret::new(config::get_jwt_secret()))
      // Store the collector
      .data(collector)
      // Enable logger
      .wrap(middleware::Logger::default())
      // Configure error handlers
      .app_data(web::JsonConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::FormConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::PathConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::QueryConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      // Load all routes
      .service(
        web::scope(&format!("/api/v1/collector/{}", collector.to_number()))
          .service(web::scope("/auth").route("", web::get().to(handlers::auth::get_me)))
          .service(
            web::scope("/elections")
              .route("", web::post().to(handlers::election::create_and_initialize_election))
              .service(
                web::scope("/{election_id}")
                  .route(
                    "/parameters",
                    web::get().to(handlers::election::get_election_parameters),
                  )
                  .service(
                    web::scope("/questions").service(
                      web::scope("/{question_id}")
                        .route(
                          "/parameters",
                          web::get().to(handlers::election::get_question_parameters),
                        )
                        .route(
                          "/cancelation",
                          web::get().to(handlers::election::get_cancelation_shares),
                        )
                        .route("/verification", web::post().to(handlers::verification::verify_ballot))
                        .route(
                          "/verification/ws/{user_id}",
                          web::get().to(handlers::verification::verify_ballot_websocket),
                        ),
                    ),
                  ),
              ),
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
  let private_key_file =
    config::get_key_file().ok_or_else(|| anyhow::anyhow!("KEY_FILE environment variable not set"))?;

  let cert_file = config::get_cert_file().ok_or_else(|| anyhow::anyhow!("CERT_FILE environment variable not set"))?;

  let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

  // Load private key and certificate chain, then validate our SSL configuration
  acceptor.set_private_key_file(private_key_file, SslFiletype::PEM)?;
  acceptor.set_certificate_chain_file(cert_file)?;
  acceptor.check_private_key()?;

  Ok(acceptor)
}
