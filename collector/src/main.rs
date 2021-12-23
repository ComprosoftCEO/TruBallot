use actix_web::client::Client;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use http::Uri;
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use serde::Serialize;
use simple_logger::SimpleLogger;
use std::str::FromStr;
use structopt::StructOpt;
use uuid_b64::UuidB64 as Uuid;

use evoting_collector::auth::{CollectorToken, JWTSecret, Permission};
use evoting_collector::config::{self, EnvPrefix};
use evoting_collector::db;
use evoting_collector::errors::{ClientRequestError, ServiceError};
use evoting_collector::handlers;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  // Parse ".env" configuration files and command-line arguments
  config::load_environment_from_env_files();
  config::Opt::from_args().update_environment();

  // Configure the logger system
  SimpleLogger::new().init()?;
  if cfg!(debug_assertions) {
    log::set_max_level(LevelFilter::Debug);
  } else {
    log::set_max_level(LevelFilter::Info);
  }

  // Let the mediator know about this collector
  register_collector_with_mediator().await?;

  // Database connection pool and web server
  let connection_pool = db::establish_new_connection_pool()?;
  let mut server = HttpServer::new(move || {
    App::new()
      // Connect to database
      .data(connection_pool.clone())
      // Encryption secret for JSON Web Token
      .data(JWTSecret::new(config::get_jwt_secret()))
      // Enable logger
      .wrap(middleware::Logger::default())
      // Configure error handlers
      .app_data(web::JsonConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::FormConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::PathConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      .app_data(web::QueryConfig::default().error_handler(|err, _req| ServiceError::from(err).into()))
      // Load all routes
      .service(
        web::scope("/api/v1/collector")
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
/// Register the collector with the mediator
///  -This is done EVERY TIME the program is run
///
async fn register_collector_with_mediator() -> anyhow::Result<()> {
  // Data needed to register an election
  let c = config::get_collector();
  let id = config::get_id().ok_or_else(|| anyhow::anyhow!("{} environment variable not set", c.env_prefix("ID")))?;
  let name = config::get_name();
  let is_secure = config::use_https();

  // Build the URI using the host and port
  let private_base_uri = Uri::from_str(&format!("//{}:{}", config::get_host(), config::get_port()))
    .map_err(|e| anyhow::anyhow!("Invalid URI: {}", e))?;

  // Url to access the mediator
  let mediator_url =
    config::get_mediator_url().ok_or_else(|| anyhow::anyhow!("MEDIATOR_URL environment variable not set"))?;

  #[derive(Serialize)]
  #[serde(rename_all = "camelCase")]
  struct CreateUpdateCollectorData {
    id: Uuid,
    name: String,
    #[serde(with = "http_serde::uri")]
    private_base_uri: Uri,
    is_secure: bool,
  }

  // Build thr request
  let register_collector_data = CreateUpdateCollectorData {
    id,
    name,
    private_base_uri,
    is_secure,
  };

  // Encode the JSON web token (Requires special permissions to manage collector)
  let jwt_encoding_key = JWTSecret::new(config::get_jwt_secret()).get_encoding_key();
  let jwt_token = CollectorToken::new(&[Permission::CanLogin, Permission::ManageCollector])
    .encode(&jwt_encoding_key)
    .map_err(|e| anyhow::anyhow!("encode JWT token: {}", e))?;

  // Send the request
  let mediator_request = Client::builder()
    .disable_timeout()
    .bearer_auth(jwt_token)
    .finish()
    .post(format!("{}/api/v1/mediator/collectors", mediator_url))
    .send_json(&register_collector_data);

  // Make sure we got a success response!
  Ok(
    ClientRequestError::handle_empty(mediator_request)
      .await
      .map_err(|e| anyhow::anyhow!("register with the mediator: {:?}", e))?,
  )
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
