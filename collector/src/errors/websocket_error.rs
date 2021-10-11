/// Handles any errors related to websockets
#[derive(Debug)]
pub enum WebsocketError {
  RequestError(actix_web::Error),
}

impl From<actix_web::Error> for WebsocketError {
  fn from(error: actix_web::Error) -> Self {
    WebsocketError::RequestError(error)
  }
}
