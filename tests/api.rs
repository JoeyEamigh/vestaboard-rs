#[cfg(feature = "local")]
use vestaboard::local::*;
#[cfg(feature = "rw")]
use vestaboard::rw::*;
#[cfg(feature = "subscription")]
use vestaboard::subscription::*;
#[cfg(any(feature = "rw", feature = "subscription", feature = "local"))]
use vestaboard::*;

// note - even though Vestaboard<T, ROWS, COLS> has defaults for ROWS and COLS,
// a type must be provided due to https://github.com/rust-lang/rust/issues/98931

#[test]
#[cfg(feature = "rw")]
fn it_creates_rw_api() {
  let config = TestConfig::new().rw.expect("no api key found for test");

  let _api: Vestaboard<RWConfig> = Vestaboard::new_rw_api(config);
}

#[test]
#[cfg(feature = "subscription")]
fn it_creates_subscription_api() {
  let config = TestConfig::new().subscription.expect("no api key found for test");

  let _api: Vestaboard<SubscriptionConfig> = Vestaboard::new_subscription_api(config);
}

#[test]
#[cfg(feature = "local")]
fn it_creates_local_api() {
  let config = TestConfig::new().local.expect("no api key found for test");

  let _api: Vestaboard<LocalConfig> = Vestaboard::new_local_api(config);
}

#[cfg(any(feature = "rw", feature = "subscription", feature = "local"))]
struct TestConfig {
  #[cfg(feature = "rw")]
  rw: Option<RWConfig>,
  #[cfg(feature = "subscription")]
  subscription: Option<SubscriptionConfig>,
  #[cfg(feature = "local")]
  local: Option<LocalConfig>,
}

#[cfg(any(feature = "rw", feature = "subscription", feature = "local"))]
impl TestConfig {
  fn new() -> Self {
    dotenv::dotenv().ok();

    #[cfg(feature = "rw")]
    let rw = if let Ok(read_write_key) = std::env::var("RW_API_KEY") {
      Some(RWConfig { read_write_key })
    } else {
      None
    };

    #[cfg(feature = "subscription")]
    let subscription = if let (Ok(api_key), Ok(api_secret)) = (
      std::env::var("SUBSCRIPTION_API_KEY"),
      std::env::var("SUBSCRIPTION_API_SECRET"),
    ) {
      Some(SubscriptionConfig { api_key, api_secret })
    } else {
      None
    };

    #[cfg(feature = "local")]
    let local =
      if let (Ok(api_key), Ok(ip_address)) = (std::env::var("LOCAL_API_KEY"), std::env::var("LOCAL_DEVICE_IP")) {
        Some(LocalConfig {
          api_key,
          ip_address: ip_address.parse().unwrap(),
        })
      } else {
        None
      };

    TestConfig {
      #[cfg(feature = "rw")]
      rw,
      #[cfg(feature = "subscription")]
      subscription,
      #[cfg(feature = "local")]
      local,
    }
  }
}
