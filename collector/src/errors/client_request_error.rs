use actix_http::{encoding::Decoder, Payload, PayloadStream};
use awc::error::{JsonPayloadError, SendRequestError};
use awc::http::StatusCode;
use awc::ClientResponse;
use serde::de::DeserializeOwned;
use std::future::Future;

use crate::errors::ErrorResponse;

/// Enum with all of the things that can go wrong when making a client request
#[derive(Debug)]
pub enum ClientRequestError {
  SendError(SendRequestError),
  ResponseError(ErrorResponse),
  JSONError(JsonPayloadError),
  UnknownError(StatusCode),
}

impl ClientRequestError {
  /// Handle the response from a client and parse the JSON body
  ///
  /// Returns an error if any of these steps fail
  pub async fn handle<T: DeserializeOwned>(
    input: impl Future<Output = Result<ClientResponse<Decoder<Payload<PayloadStream>>>, SendRequestError>>,
  ) -> Result<T, Self> {
    match input.await {
      Err(e) => Err(Self::SendError(e)),
      Ok(mut response) => {
        // Try to deserialize the data if the request is ok
        let status = response.status();
        if response.status() == StatusCode::OK {
          return response.json::<T>().await.map_err(|e| Self::JSONError(e));
        }

        // Otherwise, possibly parse an error response
        //  If the server didn't return an error response, then just return the status code
        let error_response = response
          .json::<ErrorResponse>()
          .await
          .map_err(|_| Self::UnknownError(status))?;

        Err(Self::ResponseError(error_response))
      }
    }
  }

  /// Handle the response from a client, but don't parse the JSON body
  ///
  /// Returns an error if any of these steps fail
  pub async fn handle_empty(
    input: impl Future<Output = Result<ClientResponse<Decoder<Payload<PayloadStream>>>, SendRequestError>>,
  ) -> Result<(), Self> {
    match input.await {
      Err(e) => Err(Self::SendError(e)),
      Ok(mut response) => {
        // Make sure the request is okay
        let status = response.status();
        if response.status() == StatusCode::OK {
          return Ok(());
        }

        // Otherwise, possibly parse an error response
        //  If the server didn't return an error response, then just return the status code
        let error_response = response
          .json::<ErrorResponse>()
          .await
          .map_err(|_| Self::UnknownError(status))?;

        Err(Self::ResponseError(error_response))
      }
    }
  }
}
