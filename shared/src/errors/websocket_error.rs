use actix_codec::Framed;
use awc::error::{WsClientError, WsProtocolError};
use awc::http::StatusCode;
use awc::ws::{CloseCode, Codec, Frame, Message, WebsocketsRequest};
use awc::BoxedSocket;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::ErrorResponse;

/// Handles any errors related to websockets
#[derive(Debug)]
pub enum WebsocketError {
  RequestError(actix_web::Error),
  ClientError(WsClientError),
  ProtocolError(WsProtocolError),
  JSONError(serde_json::Error),
  UnexpectedFrame(Frame),
  WebsocketClosed,
  ResponseError(ErrorResponse),
  UnknownError(StatusCode),
}

impl WebsocketError {
  /// Attempts to connect to the websocket
  /// Returns the stream on success or an error
  pub async fn connect(input: WebsocketsRequest) -> Result<Framed<BoxedSocket, Codec>, Self> {
    let (mut response, stream) = match input.connect().await {
      Err(e) => return Err(Self::ClientError(e)),
      Ok(resp) => resp,
    };

    // Try to deserialize the data if the request is ok
    let status = response.status();
    if response.status() == StatusCode::SWITCHING_PROTOCOLS {
      return Ok(stream);
    }

    // Otherwise, possibly parse an error response
    //  If the server didn't return an error response, then just return the status code
    let error_response = response
      .json::<ErrorResponse>()
      .await
      .map_err(|_| Self::UnknownError(status))?;

    Err(Self::ResponseError(error_response))
  }

  /// Serialize and send JSON data through the steam
  pub async fn send_json<T: Serialize>(stream: &mut Framed<BoxedSocket, Codec>, data: &T) -> Result<(), Self> {
    let json_string = serde_json::to_string(data).map_err(|e| Self::JSONError(e))?;
    stream
      .send(Message::Text(json_string))
      .await
      .map_err(|e| Self::ProtocolError(e))
  }

  /// Attempts to read the next websocket packet as a JSON data structure.
  /// Returns the data on success or closes the connection on error
  pub async fn read_json<T: DeserializeOwned>(stream: &mut Framed<BoxedSocket, Codec>) -> Result<T, Self> {
    // Read the next frame, closing the connection on any errors
    let frame = match stream.next().await {
      None => return Err(Self::WebsocketClosed),
      Some(Err(e)) => {
        stream
          .send(Message::Close(Some((CloseCode::Error, format!("{}", e)).into())))
          .await
          .map_err(|e| Self::ProtocolError(e))?;

        return Err(Self::ProtocolError(e));
      },
      Some(Ok(resp)) => resp,
    };

    // Make sure the frame is text
    let text = match frame {
      Frame::Text(text) => text,
      other => return Err(Self::UnexpectedFrame(other)),
    };

    // Parse the JSON
    serde_json::from_slice::<T>(text.as_ref()).map_err(|e| Self::JSONError(e))
  }
}

impl From<actix_web::Error> for WebsocketError {
  fn from(error: actix_web::Error) -> Self {
    WebsocketError::RequestError(error)
  }
}

impl From<WsClientError> for WebsocketError {
  fn from(error: WsClientError) -> Self {
    WebsocketError::ClientError(error)
  }
}
