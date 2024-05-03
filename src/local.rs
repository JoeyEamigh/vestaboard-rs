//! # local api (requires the `local` feature)
//! the local api is used to send messages to a single Vestaboard on your local network. \
//! requires the local api enabled on your vestaboard and an api key
//!
//! ## new
//! ```
//! fn new_local_api(config: LocalConfig) -> Self
//! ```
//!
//! ## args
//! ```
//! LocalConfig {
//!   api_key: "<YOUR_LOCAL_API_KEY>",
//!   ip_address: "<YOUR_VESTABOARD_IP_ADDRESS>".parse().expect("failed to parse ip address"),
//! }
//! ```
//!
//! ## static methods
//! ```
//! async fn get_local_api_key(
//!    ip_address: Option<std::net::IpAddr>,
//!    local_enablement_token: Option<String>,
//! ) -> Result<String, LocalApiError>
//! ```
//!
//! ## methods
//! ```
//! async fn read(&self) -> Result<BoardData<ROWS, COLS>, LocalApiError>
//! async fn write(&self, message: BoardData<ROWS, COLS>) -> Result<(), LocalApiError>
//! ```
//!
//! ## types
//! - [`LocalConfig`] is the config type for the local api
//! - [`LocalApiError`] is the error enum for the local api
//!
//! ## example
//! ```
//! let config = LocalConfig {
//!  api_key: "<YOUR_LOCAL_API_KEY>",
//!  ip_address: "<YOUR_VESTABOARD_IP_ADDRESS>".parse().expect("failed to parse ip address"),
//! };
//!
//! // note that a type must be included because of <https://github.com/rust-lang/rust/issues/98931>
//! let api: Vestaboard<LocalConfig> = Vestaboard::new_local_api(config);
//! ```
//!
//! <https://docs.vestaboard.com/docs/local-api/introduction>

use serde::Deserialize;
use thiserror::Error;

use crate::{board::BoardData, Vestaboard};

const LOCAL_ENABLEMENT_TOKEN_HEADER: &str = "X-Vestaboard-Local-Api-Enablement-Token";
const LOCAL_API_KEY_HEADER: &str = "X-Vestaboard-Local-Api-Key";

const LOCAL_DEVICE_PORT: u16 = 7000;

const LOCAL_API_ENABLEMENT_URI: &str = "/local-api/enablement";
const LOCAL_API_MESSAGE_URI: &str = "/local-api/message";

/// configuration object for the Vestaboard local api \
/// <https://docs.vestaboard.com/docs/local-api/introduction>
///
/// note that Vestaboard recommends using IPV4
#[derive(Debug, Clone)]
pub struct LocalConfig {
  /// the local api key for your Vestaboard \
  /// <<https://docs.vestaboard.com/docs/local-api/authentication>>
  pub api_key: String,
  /// the IP address of your Vestaboard \
  /// note that Vestaboard recommends using IPV4
  pub ip_address: std::net::IpAddr,
}

impl<const ROWS: usize, const COLS: usize> Vestaboard<LocalConfig, ROWS, COLS> {
  /// create a new [`Vestaboard`] instance for a local Vestaboard. \
  /// requires the local api enabled on your Vestaboard and an api key
  ///
  /// # args
  /// ```
  /// LocalConfig {
  ///   api_key: "<YOUR_LOCAL_API_KEY>",
  ///   ip_address: "<YOUR_VESTABOARD_IP_ADDRESS>",
  /// }
  /// ```
  ///
  /// # returns
  /// a new [`Vestaboard`] instance
  ///
  /// # panics
  /// panics if the `api_key` cannot be parsed
  ///
  ///
  /// <https://docs.vestaboard.com/docs/local-api/introduction>
  pub fn new_local_api(config: LocalConfig) -> Self {
    use std::str::FromStr;

    let headers = reqwest::header::HeaderMap::from_iter([
      (
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
      ),
      (
        reqwest::header::HeaderName::from_str(LOCAL_API_KEY_HEADER).unwrap(),
        reqwest::header::HeaderValue::from_str(&config.api_key).expect("failed to parse api key"),
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
  /// the current message on the Vestaboard as a [`BoardData<ROWS, COLS>`]
  ///
  /// # errors
  /// - [`ReqwestError`](LocalApiError::Reqwest) if there is an error with the reqwest client
  /// - [`ApiError`](LocalApiError::ApiError) if there is an error with the local api
  pub async fn read(&self) -> Result<BoardData<ROWS, COLS>, LocalApiError> {
    let url = format!(
      "http://{}:{}{}",
      self.config.ip_address, LOCAL_DEVICE_PORT, LOCAL_API_MESSAGE_URI
    );
    let res = self.client.get(url).send().await?;

    if !res.status().is_success() {
      return Err(LocalApiError::ApiError(res.text().await?));
    }

    Ok(res.json().await?)
  }

  /// write a message to the Vestaboard
  ///
  /// # args
  /// - `message`: the [`BoardData<ROWS, COLS>`] message to write to the Vestaboard
  ///
  /// # errors
  /// - [`ReqwestError`](LocalApiError::Reqwest) if there is an error with the reqwest client
  /// - [`ApiError`](LocalApiError::ApiError) if there is an error with the local api
  pub async fn write(&self, message: BoardData<ROWS, COLS>) -> Result<(), LocalApiError> {
    let url = format!(
      "http://{}:{}{}",
      self.config.ip_address, LOCAL_DEVICE_PORT, LOCAL_API_MESSAGE_URI
    );
    let res = self.client.post(url).json(&message).send().await?;

    if !res.status().is_success() {
      Err(LocalApiError::ApiError(res.text().await?))
    } else {
      Ok(())
    }
  }

  /// static method to get the local api key for your Vestaboard. \
  /// requires a local api enablement token.
  ///
  /// request one here: <https://www.vestaboard.com/local-api>
  ///
  /// note that this api will only return the api key once per enablement token, so be sure to store it.
  ///
  /// # args
  /// ```
  /// ip_address: Option<std::net::IpAddr> // the ip address of your Vestaboard
  /// local_enablement_token: Option<String> // the local api enablement token
  /// ```
  ///
  /// if the args are not provided, the function will look for the following environment variables:
  /// - `LOCAL_DEVICE_IP` for the ip address
  /// - `LOCAL_ENABLEMENT_TOKEN` for the local api enablement token
  ///
  /// # returns
  /// the local api key for your Vestaboard as a [`String`]
  ///
  /// # errors
  /// - [`ReqwestError`](LocalApiError::Reqwest) if there is an error with the reqwest client
  /// - [`MissingHeader`](LocalApiError::MissingHeader) if the `local_enablement_token` or `device_ip` is missing
  /// - [`InvalidIp`](LocalApiError::InvalidIp) if the `device_ip` is not a valid IP address
  /// - [`ApiError`](LocalApiError::ApiError) if there is an error with the local api
  ///
  /// <https://docs.vestaboard.com/docs/local-api/authentication>
  pub async fn get_local_api_key(
    ip_address: Option<std::net::IpAddr>,
    local_enablement_token: Option<String>,
  ) -> Result<String, LocalApiError> {
    let token = if let Some(token) = local_enablement_token {
      token
    } else if let Ok(token) = std::env::var("LOCAL_ENABLEMENT_TOKEN") {
      token
    } else {
      return Err(LocalApiError::MissingHeader {
        name: "local_enablement_token".to_string(),
        env_var: "LOCAL_ENABLEMENT_TOKEN".to_string(),
      });
    };

    let ip = if let Some(ip) = ip_address {
      ip
    } else if let Ok(ip) = std::env::var("LOCAL_DEVICE_IP") {
      if let Ok(ip) = ip.parse::<std::net::IpAddr>() {
        ip
      } else {
        return Err(LocalApiError::InvalidIp);
      }
    } else {
      return Err(LocalApiError::MissingHeader {
        name: "device_ip".to_string(),
        env_var: "LOCAL_DEVICE_IP".to_string(),
      });
    };

    let headers = reqwest::header::HeaderMap::from_iter([
      (
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
      ),
      (
        reqwest::header::HeaderName::from_static(LOCAL_ENABLEMENT_TOKEN_HEADER),
        reqwest::header::HeaderValue::from_str(&token).expect("failed to parse local enablement token"),
      ),
    ]);

    let client = reqwest::Client::builder()
      .default_headers(headers)
      .user_agent(format!("vestaboard-rs/{}", env!("CARGO_PKG_VERSION")))
      .build()
      .expect("failed to build reqwest client");

    let url = format!("http://{}:{}{}", ip, LOCAL_DEVICE_PORT, LOCAL_API_ENABLEMENT_URI);
    let res = client.post(url).send().await?;

    let body: LocalApiEnablementResponse = res.json().await?;

    if let Some(api_key) = body.api_key {
      Ok(api_key)
    } else {
      Err(LocalApiError::ApiError(body.message))
    }
  }
}

/// response type for the local api enablement request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalApiEnablementResponse {
  /// the success or error message
  message: String,
  /// the local api key for your Vestaboard
  api_key: Option<String>,
}

/// error type for the Vestaboard local api
/// - [`ReqwestError`](LocalApiError::Reqwest) if there is an error with the reqwest client
/// - [`MissingHeader`](LocalApiError::MissingHeader) if the `local_enablement_token` or `device_ip` is missing
/// - [`InvalidIp`](LocalApiError::InvalidIp) if the `device_ip` is not a valid IP address
/// - [`ApiError`](LocalApiError::ApiError) if there is an error with the local api
#[derive(Error, Debug)]
pub enum LocalApiError {
  /// reqwest error, see wrapped reqwest::Error for more details
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  /// missing header error - see `name` for the missing header and `env_var` for the environment variable that can be set instead of passing the value
  #[error("missing header `{name:?}`. pass the value or set the `{env_var:?}` environment variable.")]
  MissingHeader { name: String, env_var: String },
  /// invalid ip address for local device
  #[error("invalid ip address for local device")]
  InvalidIp,
  /// api error with wrapped message
  #[error("api error: {0}")]
  ApiError(String),
}
