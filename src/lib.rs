#![doc = include_str!("../README.md")]

pub mod board;

#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "rw")]
pub mod rw;
#[cfg(feature = "subscription")]
pub mod subscription;
#[cfg(feature = "parser")]
pub mod vbml;

// reexports
pub use board::{BoardData, CharacterCode};
#[cfg(feature = "local")]
pub use local::{LocalApiError, LocalConfig};
#[cfg(feature = "rw")]
pub use rw::{RWApiError, RWApiReadMessage, RWApiWriteResponse, RWConfig};
#[cfg(feature = "subscription")]
pub use subscription::{SubscriptionApiError, SubscriptionConfig, SubscriptionMessageResponse, SubscriptionsList};
#[cfg(feature = "parser")]
pub use vbml::Vbml;

/// the main struct for interacting with the Vestaboard api. \
/// can interact with the r/w api, the subscription api, or the local api.
///
/// # type parameters
/// - `T`: the config type for the api. this determines which api will be used to interact with the Vestaboard
/// - `ROWS`: the number of rows in the board, defaults to the flagship board size
/// - `COLS`: the number of columns in the board, defaults to the flagship board size
///
/// # notes
/// - when initializing the Vestaboard struct, a type must be provided due to <https://github.com/rust-lang/rust/issues/98931>
///
/// # read/write api (requires the `rw` feature)
/// the read/write api is used to send messages to a single Vestaboard. the read/write api must
/// be enabled for the Vestaboard.
///
/// ## new
/// ```
/// fn new_rw_api(config: RWConfig) -> Self
/// ```
///
/// ## args
/// ```
/// RWConfig {
///   read_write_key: "<YOUR_RW_API_KEY>",
/// }
/// ```
///
/// ## methods
/// ```
/// async fn read(&self) -> Result<RWApiReadMessage, RWApiError>
/// async fn write(&self, message: BoardData<ROWS, COLS>) -> Result<String, RWApiError> // returns the message id
/// ```
///
/// ## types
/// - [`RWConfig`] is the config type for the read/write api
/// - [`RWApiReadMessage`] is the response type for the read method
/// - [`RWApiWriteResponse`] is the response type for the write method
/// - [`RWApiError`] is the error enum for the read/write api
///
///
/// <https://docs.vestaboard.com/docs/read-write-api/introduction>
///
///
/// # subscription api (requires the `subscription` feature)
/// the subscription api is used to send messages to multiple Vestaboards. requires a valid
/// installable with access to the Vestaboard, and the api key and secret for that installable.
///
/// ## new
/// ```
/// fn new_subscription_api(config: SubscriptionConfig) -> Self
/// ```
///
/// ## args
/// ```
/// SubscriptionConfig {
///   api_key: "<YOUR_SUBSCRIPTION_API_KEY>",
///   api_secret: "<YOUR_SUBSCRIPTION_API_SECRET>",
/// }
/// ```
///
/// ## methods
/// ```
/// async fn get_subscriptions(&self) -> Result<SubscriptionsList, SubscriptionApiError>
/// async fn write(&self, subscription_id: &str, message: BoardData<ROWS, COLS>) -> Result<SubscriptionMessageResponse, SubscriptionApiError>
/// ```
///
/// ## types
/// - [`SubscriptionConfig`] is the config type for the subscription api
/// - [`SubscriptionsList`] is the response type for the get_subscriptions method
/// - [`SubscriptionMessageResponse`] is the response type for the write method
/// - [`SubscriptionApiError`] is the error enum for the subscription api
///
/// <https://docs.vestaboard.com/docs/subscription-api/introduction>
///
///
/// # local api (requires the `local` feature)
/// the local api is used to send messages to a single Vestaboard on your local network. \
/// requires the local api enabled on your vestaboard and an api key
///
/// ## new
/// ```
/// fn new_local_api(config: LocalConfig) -> Self
/// ```
///
/// ## args
/// ```
/// LocalConfig {
///   api_key: "<YOUR_LOCAL_API_KEY>",
///   ip_address: "<YOUR_VESTABOARD_IP_ADDRESS>".parse().expect("failed to parse ip address"),
/// }
/// ```
///
/// ## static methods
/// ```
/// async fn get_local_api_key(
///    ip_address: Option<std::net::IpAddr>,
///    local_enablement_token: Option<String>,
/// ) -> Result<String, LocalApiError>
/// ```
///
/// ## methods
/// ```
/// async fn read(&self) -> Result<BoardData<ROWS, COLS>, LocalApiError>
/// async fn write(&self, message: BoardData<ROWS, COLS>) -> Result<(), LocalApiError>
/// ```
///
/// ## types
/// - [`LocalConfig`] is the config type for the local api
/// - [`LocalApiError`] is the error enum for the local api
///
/// <https://docs.vestaboard.com/docs/local-api/introduction>
///
/// # api docs
///
/// <https://docs.vestaboard.com/>
#[cfg(any(feature = "rw", feature = "subscription", feature = "local"))]
#[derive(Debug, Clone)]
pub struct Vestaboard<T, const ROWS: usize = { board::FLAGSHIP_ROWS }, const COLS: usize = { board::FLAGSHIP_COLS }> {
  client: reqwest::Client,
  #[allow(dead_code)] // subscription api complains but is used for type inference
  config: T,
}
