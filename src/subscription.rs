//! # Vestaboard subscription api (requires the `subscription` feature)
//! the subscription api is used to send messages to multiple Vestaboards. requires a valid
//! installable with access to the Vestaboard, and the api key and secret for that installable.
//!
//! ## new
//! ```
//! fn new_subscription_api(config: SubscriptionConfig) -> Self
//! ```
//!
//! ## args
//! ```
//! SubscriptionConfig {
//!   api_key: "<YOUR_SUBSCRIPTION_API_KEY>",
//!   api_secret: "<YOUR_SUBSCRIPTION_API_SECRET>",
//! }
//! ```
//!
//! ## methods
//! ```
//! async fn get_subscriptions(&self) -> Result<SubscriptionsList, SubscriptionApiError>
//! async fn write(&self, subscription_id: &str, message: BoardData<ROWS, COLS>) -> Result<SubscriptionMessageResponse, SubscriptionApiError>
//! ```
//!
//! ## types
//! - [`SubscriptionConfig`] is the config type for the subscription api
//! - [`SubscriptionsList`] is the response type for the get_subscriptions method
//! - [`SubscriptionMessageResponse`] is the response type for the write method
//! - [`SubscriptionApiError`] is the error enum for the subscription api
//!
//! ## example
//! ```
//! let config = SubscriptionConfig {
//!   api_key: "<YOUR_SUBSCRIPTION_API_KEY>",
//!   api_secret: "<YOUR_SUBSCRIPTION_API_SECRET>",
//! };
//!
//! // note that a type must be included because of <https://github.com/rust-lang/rust/issues/98931>
//! let api: Vestaboard<SubscriptionConfig> = Vestaboard::new_subscription_api(config);
//! ```
//!
//! <https://docs.vestaboard.com/docs/subscription-api/introduction>

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{BoardData, Vestaboard};

const SUBSCRIPTION_API_KEY_HEADER: &str = "X-Vestaboard-Api-Key";
const SUBSCRIPTION_API_SECRET_HEADER: &str = "X-Vestaboard-Api-Secret";

const LIST_SUBSCRIPTIONS_URI: &str = "https://subscriptions.vestaboard.com/subscriptions";
// const SEND_MESSAGE_URI: &str = "https://subscriptions.vestaboard.com/subscriptions/{}/message";

/// configuration object for the Vestaboard Subscription API
///
/// <https://docs.vestaboard.com/docs/subscription-api/introduction>
#[derive(Debug, Clone)]
pub struct SubscriptionConfig {
  /// the api key of your installable
  pub api_key: String,
  /// the api secret of your installable
  pub api_secret: String,
}

impl<const ROWS: usize, const COLS: usize> Vestaboard<SubscriptionConfig, ROWS, COLS> {
  /// create a new [`Vestaboard`] instance Vestaboards managed by the subscription api. \
  /// requires a valid installable with access to the Vestaboard, and the api key and secret for that installable
  ///
  /// # args
  /// ```
  /// SubscriptionConfig {
  ///   api_key: "<YOUR_SUBSCRIPTION_API_KEY>",
  ///   api_secret: "<YOUR_SUBSCRIPTION_API_SECRET>",
  /// }
  /// ```
  ///
  /// # returns
  /// a new [`Vestaboard`] instance
  ///
  ///
  /// <https://docs.vestaboard.com/docs/subscription-api/introduction>
  pub fn new_subscription_api(config: SubscriptionConfig) -> Self {
    use std::str::FromStr;

    let headers = reqwest::header::HeaderMap::from_iter([
      (
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
      ),
      (
        reqwest::header::HeaderName::from_str(SUBSCRIPTION_API_KEY_HEADER).unwrap(),
        reqwest::header::HeaderValue::from_str(&config.api_key).expect("failed to parse api key"),
      ),
      (
        reqwest::header::HeaderName::from_str(SUBSCRIPTION_API_SECRET_HEADER).unwrap(),
        reqwest::header::HeaderValue::from_str(&config.api_secret).expect("failed to parse api secret"),
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

  /// get a list of Vestaboards that this installable has access to
  ///
  /// # returns
  /// a list of Vestaboards that this installable has access to as a [`SubscriptionsList`]
  ///
  /// # errors
  /// - [`SubscriptionApiError::Reqwest`] if there was an error sending the request
  /// - [`SubscriptionApiError::Deserialize`] if there was an error parsing the response
  /// - [`SubscriptionApiError::ApiError`] if there was an error with the subscription api
  pub async fn get_subscriptions(&self) -> Result<SubscriptionsList, SubscriptionApiError> {
    let res = self.client.get(LIST_SUBSCRIPTIONS_URI).send().await?;

    if !res.status().is_success() {
      return Err(SubscriptionApiError::ApiError(res.text().await?));
    }

    Ok(res.json::<SubscriptionsList>().await?)
  }

  /// send a message to a subscribed Vestaboard
  ///
  /// # args
  /// - `subscription_id`: the id of the subscription to send the message to
  /// - `message`: the message to send to the Vestaboard as a [`BoardData<ROWS, COLS>`]
  ///
  /// # returns
  /// the response from the Vestaboard Subscription API as a [`SubscriptionMessageResponse`]
  ///
  /// # errors
  /// - [`SubscriptionApiError::Reqwest`] if there was an error sending the request
  /// - [`SubscriptionApiError::Deserialize`] if there was an error parsing the response
  /// - [`SubscriptionApiError::ApiError`] if there was an error with the subscription api
  pub async fn write(
    &self,
    subscription_id: &str,
    message: BoardData<ROWS, COLS>,
  ) -> Result<SubscriptionMessageResponse, SubscriptionApiError> {
    let message = SubscriptionMessage { characters: message };

    let res = self
      .client
      .post(&format!(
        "https://subscriptions.vestaboard.com/subscriptions/{}/message",
        subscription_id
      ))
      .json(&message)
      .send()
      .await?;

    if !res.status().is_success() {
      return Err(SubscriptionApiError::ApiError(res.text().await?));
    }

    Ok(res.json::<SubscriptionMessageResponse>().await?)
  }
}

/// message to send to a subscribed Vestaboard
#[derive(Debug, Clone, Serialize)]
struct SubscriptionMessage<const ROWS: usize, const COLS: usize> {
  /// raw message to send to the Vestaboard
  characters: BoardData<ROWS, COLS>,
}

/// response from the Vestaboard Subscription API when sending a message
#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionMessageResponse {
  /// the id of the message
  pub id: String,
  /// the unix timestamp in milliseconds that the message was created (as a string for some reason)
  pub created: String,
  /// whether the message is muted
  pub muted: bool,
}

/// a Vestaboard that this installable has access to
#[derive(Debug, Clone, Deserialize)]
pub struct Subscription {
  /// the id of the subscription
  pub id: String,
  /// the id of the Vestaboard
  pub board_id: String,
}

/// list of subscribed Vestaboards that this installable has access to
#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionsList(pub Vec<Subscription>);

/// error type for the Vestaboard subscription api
#[derive(Error, Debug)]
pub enum SubscriptionApiError {
  /// reqwest error, see wrapped reqwest::Error for more details
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  /// failed to deserialize api response, see wrapped serde_json::Error for more details
  #[error("failed to parse response: {0}")]
  Deserialize(#[from] serde_json::Error),
  /// api error with wrapped message
  #[error("api error: {0}")]
  ApiError(String),
}
