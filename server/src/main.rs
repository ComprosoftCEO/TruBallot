use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

use evoting_server::auth;
use evoting_server::config;
use evoting_server::db;
use evoting_server::errors::ServiceError;
use evoting_server::handlers;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  // Parse ".env" configuration files and command-line arguments
  config::load_environment_from_env_files();
  config::Opt::from_args().update_environment();

  // Configure the logger system
  SimpleLogger::new().init().unwrap();
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
      // Enable logger
      .wrap(middleware::Logger::default())
      // Configure error handlers
      .app_data(web::JsonConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::FormConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::PathConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::QueryConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      // Load all routes
      .service(
        web::scope("/api/v1")
          .service(
            web::scope("/auth")
              .route("", web::get().to(handlers::auth::get_me))
              .route("/login", web::post().to(handlers::auth::login))
              .route("/refresh", web::post().to(handlers::auth::refresh)),
          )
          .service(
            web::scope("/account")
              .route("", web::patch().to(handlers::account::update_account))
              .route("/register", web::post().to(handlers::account::register_account))
              .route("/password", web::put().to(handlers::account::update_password)),
          )
          .service(
            web::scope("/elections")
              .route("", web::get().to(handlers::election::all_elections))
              .route("", web::post().to(handlers::election::create_election))
              .route(
                "/access-code",
                web::get().to(handlers::election::get_election_by_access_code),
              )
              .service(
                web::scope("/{election_id}")
                  .route("", web::get().to(handlers::election::get_election))
                  .route("", web::patch().to(handlers::election::update_election))
                  .route("", web::delete().to(handlers::election::delete_election))
                  .route(
                    "/parameters",
                    web::get().to(handlers::election::get_election_parameters),
                  )
                  .route("/results", web::get().to(handlers::election::get_election_results))
                  .service(
                    web::scope("/registration")
                      .route("", web::post().to(handlers::registration::register_for_election))
                      .route("", web::delete().to(handlers::registration::unregister_from_election))
                      .route("", web::put().to(handlers::registration::open_registration)),
                  )
                  .service(
                    web::scope("/voting")
                      .route("", web::post().to(handlers::voting::initialize_voting))
                      .route("", web::delete().to(handlers::voting::close_voting)),
                  )
                  .service(
                    web::scope("/questions/{question_id}").route("/vote", web::post().to(handlers::voting::vote)),
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
