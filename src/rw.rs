//! # Vestaboard Read/Write api (requires the `rw` feature)
//!
//! this module contains the implementation for the Vestaboard Read/Write api. the
//! `rw` flag must be enabled to use this module. the Read/Write api is used to send
//! messages to a single Vestaboard. the read/write api must be enabled for the Vestaboard.
//!
//! ## config
//! ```
//! RWConfig {
//!  read_write_key: String,
//! }
//! ```
//!
//! ## methods
//! ```
//! async fn read(&self) -> Result<RWApiReadMessage, RWApiError>
//! async fn write(&self, message: BoardData<ROWS, COLS>) -> Result<String, RWApiError> // returns the message id
//! ```
//!
//! ## types
//! - [`RWConfig`] is the config type for the read/write api
//! - [`RWApiReadMessage`] is the response type for the read method
//! - [`RWApiWriteResponse`] is the response type for the write method
//! - [`RWApiError`] is the error enum for the read/write api
//!
//! ## example
//! ```
//! let config = RWConfig {
//!  read_write_key: "<YOUR_RW_API_KEY>",
//! };
//!
//! // note that a type must be included because of <https://github.com/rust-lang/rust/issues/98931>
//! let api: Vestaboard<RWConfig> = Vestaboard::new_rw_api(config);
//! ```
//!
//! <https://docs.vestaboard.com/docs/read-write-api/introduction>

use serde::Deserialize;
use thiserror::Error;

use crate::{BoardData, Vestaboard};

const RW_API_URI: &str = "https://rw.vestaboard.com/";
const RW_API_HEADER: &str = "X-Vestaboard-Read-Write-Key";

/// configuration object for the Vestaboard Read/Write API \
/// <https://docs.vestaboard.com/docs/read-write-api/introduction>
#[derive(Debug, Clone)]
pub struct RWConfig {
  /// the read/write key for your Vestaboard \
  /// <https://docs.vestaboard.com/docs/read-write-api/authentication>
  pub read_write_key: String,
}

impl<const ROWS: usize, const COLS: usize> Vestaboard<RWConfig, ROWS, COLS> {
  /// create a new [`Vestaboard`] instance for a read/write api enabled Vestaboard. \
  /// requires the read/write api enabled on your vestaboard and an api key
  ///
  /// # args
  /// ```
  /// RWConfig {
  ///   read_write_key: "<YOUR_RW_API_KEY>",
  /// }
  /// ```
  ///
  /// # returns
  /// a new [`Vestaboard`] instance
  ///
  ///
  /// <https://docs.vestaboard.com/docs/read-write-api/introduction>
  pub fn new_rw_api(config: RWConfig) -> Self {
    use std::str::FromStr;

    let headers = reqwest::header::HeaderMap::from_iter([
      (
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
      ),
      (
        reqwest::header::HeaderName::from_str(RW_API_HEADER).unwrap(),
        reqwest::header::HeaderValue::from_str(&config.read_write_key).expect("failed to parse read/write key"),
      ),
    ]);

    Vestaboard {
      client: reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(format!("vestaboard-rs/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .expect("failed to build reqwest client"),
      config,
    }
  }

  /// read the current message on the Vestaboard
  ///
  /// # returns
  /// the current message on the Vestaboard as a
  ///
  /// # errors
  /// - [`ReqwestError`](RWApiError::Reqwest) if there is an error with the reqwest client
  /// - [`DeserializeError`](RWApiError::Deserialize) if there is an error deserializing the response
  /// - [`ParseBoardData`](RWApiError::ParseBoardData) if there is an error parsing the message layout into a [`BoardData`]
  /// - [`ApiError`](RWApiError::ApiError) if there is an error with the r/w api
  pub async fn read(&self) -> Result<RWApiReadMessage<ROWS, COLS>, RWApiError> {
    use std::str::FromStr;

    let res = self.client.get(RW_API_URI).send().await?;

    if !res.status().is_success() {
      return Err(RWApiError::ApiError(res.text().await?));
    }

    let res = res.json::<RWApiReadResponse>().await?;
    let board = BoardData::from_str(&res.current_message.layout)?;

    Ok(RWApiReadMessage {
      layout: res.current_message.layout,
      id: res.current_message.id,
      board,
    })
  }

  /// write a message to the Vestaboard
  ///
  /// # args
  /// - `message`: the [`BoardData<ROWS, COLS>`] message to write to the Vestaboard
  ///
  /// # errors
  /// - [`ReqwestError`](RWApiError::Reqwest) if there is an error with the reqwest client
  /// - [`ApiError`](RWApiError::ApiError) if there is an error with the r/w api
  pub async fn write(&self, message: BoardData<ROWS, COLS>) -> Result<RWApiWriteResponse, RWApiError> {
    let res = self.client.post(RW_API_URI).json(&message).send().await?;

    if !res.status().is_success() {
      return Err(RWApiError::ApiError(res.text().await?));
    }

    Ok(res.json::<RWApiWriteResponse>().await?)
  }
}

/// the current message on the Vestaboard
pub struct RWApiReadMessage<const ROWS: usize, const COLS: usize> {
  /// a string representation of a [`crate::board::Board<ROWS, COLS>`]
  pub layout: String,
  /// the id of the message that is on the Vestaboard
  pub id: String,
  /// the message on the Vestaboard
  pub board: BoardData<ROWS, COLS>,
}

/// the current message of the Vestaboard
#[derive(Debug, Clone, Deserialize)]
struct RWApiRawMessage {
  /// a string representation of a [`Board<ROWS, COLS>`]
  pub layout: String,
  /// the id of the message that is on the Vestaboard
  pub id: String,
}

/// the response from the read endpoint of the Vestaboard Read/Write API
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RWApiReadResponse {
  /// the current message on the Vestaboard
  pub current_message: RWApiRawMessage,
}

#[derive(Debug, Clone, Deserialize)]
/// the response from the write endpoint of the Vestaboard Read/Write API
pub struct RWApiWriteResponse {
  /// the status of the message that was written to the Vestaboard, usually `ok`
  pub status: String,
  /// the id of the message that was written to the Vestaboard
  pub id: String,
  /// the unix timestamp in milliseconds that the message was written to the Vestaboard
  pub created: usize,
}

/// errors that can occur when using the Vestaboard Read/Write API
/// - [`RWApiError::Reqwest`] if there is an error with the reqwest client
/// - [`RWApiError::Deserialize`] if there is an error deserializing the response
#[derive(Error, Debug)]
pub enum RWApiError {
  /// reqwest error, see wrapped [`reqwest::Error`] for more details
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  /// failed to deserialize, see wrapped serde_json::Error for more details
  #[error("failed to parse response: {0}")]
  Deserialize(#[from] serde_json::Error),
  /// failed to parse the message layout into a [`BoardData`]
  #[error("failed to parse message layout: {0}")]
  ParseBoardData(#[from] crate::board::BoardError),
  /// api error with wrapped message
  #[error("api error: {0}")]
  ApiError(String),
}
