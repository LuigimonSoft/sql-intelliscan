use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

use tracing_subscriber::util::{SubscriberInitExt, TryInitError};
use tracing_subscriber::{fmt as subscriber_fmt, EnvFilter};

pub const DEFAULT_LOG_FILTER: &str = "info";
pub const LOG_FILTER_ENV_VAR: &str = "RUST_LOG";

static LOGGING_INITIALIZED: OnceLock<()> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggingStackDecision {
    pub emitter: &'static str,
    pub subscriber: &'static str,
    pub file_appender: Option<&'static str>,
    pub environment_variable: &'static str,
    pub default_filter: &'static str,
}

#[derive(Debug)]
pub enum LoggingInitError {
    Subscriber(TryInitError),
}

impl fmt::Display for LoggingInitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Subscriber(error) => {
                write!(formatter, "failed to initialize backend logging: {error}")
            }
        }
    }
}

impl Error for LoggingInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Subscriber(error) => Some(error),
        }
    }
}

pub fn logging_stack_decision() -> LoggingStackDecision {
    LoggingStackDecision {
        emitter: "tracing",
        subscriber: "tracing-subscriber",
        file_appender: None,
        environment_variable: LOG_FILTER_ENV_VAR,
        default_filter: DEFAULT_LOG_FILTER,
    }
}

pub fn build_log_filter_from_env_value(
    configured_filter: Result<String, std::env::VarError>,
) -> EnvFilter {
    configured_filter
        .ok()
        .and_then(|filter| EnvFilter::try_new(filter).ok())
        .unwrap_or_else(|| EnvFilter::new(DEFAULT_LOG_FILTER))
}

pub fn init_logging() -> Result<(), LoggingInitError> {
    if LOGGING_INITIALIZED.get().is_some() {
        return Ok(());
    }

    let filter = build_log_filter_from_env_value(std::env::var(LOG_FILTER_ENV_VAR));

    match subscriber_fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .finish()
        .try_init()
    {
        Ok(()) => {
            let _ = LOGGING_INITIALIZED.set(());
            Ok(())
        }
        Err(error) if is_global_subscriber_already_set(&error) => {
            let _ = LOGGING_INITIALIZED.set(());
            Ok(())
        }
        Err(error) => Err(LoggingInitError::Subscriber(error)),
    }
}

fn is_global_subscriber_already_set(error: &TryInitError) -> bool {
    error
        .to_string()
        .contains("global default trace dispatcher has already been set")
}
