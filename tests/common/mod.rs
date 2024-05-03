pub fn setup() {
  use tracing::metadata::LevelFilter;
  use tracing_subscriber::{
    filter::Directive, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
  };

  let default_directive = Directive::from(LevelFilter::TRACE);
  let filter_directives = if let Ok(filter) = std::env::var("RUST_LOG") {
    filter
  } else {
    "vestaboard=trace".to_string()
  };

  let filter = EnvFilter::builder()
    .with_default_directive(default_directive)
    .parse_lossy(filter_directives);
  let subscriber = tracing_subscriber::registry().with(fmt::layer().without_time().with_filter(filter));

  subscriber.init();
}
