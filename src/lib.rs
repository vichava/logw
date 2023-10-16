pub use tracing;

#[cfg(feature = "application")]
use tracing::metadata::LevelFilter;
#[cfg(feature = "application")]
use tracing_subscriber::EnvFilter;
#[cfg(feature = "application")]
use tracing_subscriber::fmt::format::{DefaultFields, Format};
#[cfg(feature = "application")]
use tracing_subscriber::fmt::SubscriberBuilder;

#[cfg(feature = "application")]
const RUST_LOG_FORMAT_ENV: &str = "RUST_LOG_FORMAT";
#[cfg(feature = "application")]
const DEFAULT_LOG_FORMAT: &str = "full";

/// Initializes tracing logging by defining a subscriber which is used to collect
/// or filter trace data. It sets the log format according to `RUST_LOG_FORMAT` environmental variable
/// and level according to `RUST_LOG` environmental variable.
///
/// RUST_LOG_FORMAT can be one of the following:
/// - full (default)
/// - compact
/// - pretty
/// - json
///
/// RUN_LOG follows the same format as the standard `log` crate
///
/// This logger can be initialized many times, but only the first initialization will have an effect.
#[cfg(feature = "application")]
pub fn init() {
    let log_format: String = std::env::var(RUST_LOG_FORMAT_ENV)
        .unwrap_or_else(|_| DEFAULT_LOG_FORMAT.to_string())
        .to_lowercase();

    // Build the base subscriber
    let subscriber_builder: SubscriberBuilder<DefaultFields, Format, EnvFilter> = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy()
        );

    // Init the subscriber with the log format from the RUST_LOG_FORMAT environment variable
    let result: Result<(), Box<dyn std::error::Error + Send + Sync>> = match log_format.as_str() {
        "compact" => subscriber_builder.compact().try_init(),
        "pretty" => subscriber_builder.pretty().try_init(),
        "json" => subscriber_builder.json().try_init(),
        _ => subscriber_builder.try_init(),
    };

    // Log the result of the subscriber initialization
    if result.is_ok() {
        tracing::info!("Logger initialized");
    }
}

#[cfg(test)]
#[cfg(feature = "application")]
mod tests {
    use std::env;
    use super::*;
    use tracing::{debug, error, info, trace, warn};

    #[test]
    fn test_log_functionality() {
        env::set_var("LOG_LEVEL", "trace");
        init();

        trace!("Test logger message");
        debug!("Test logger message");
        info!("Test logger message");
        warn!("Test logger message");
        error!("Test logger message");
    }

    #[test]
    fn test_multiple_init() {
        env::set_var("LOG_LEVEL", "trace");
        init();
        init();
        init();
    }

}