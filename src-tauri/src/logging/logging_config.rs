use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

use tracing_subscriber::util::{SubscriberInitExt, TryInitError};
use tracing_subscriber::{fmt as subscriber_fmt, EnvFilter};

use crate::config::environment::{
    current_app_environment, AppEnvironment, APP_ENVIRONMENT_ENV_VAR,
};

pub const DEFAULT_LOG_FILTER: &str = "debug";
pub const LOG_FILTER_ENV_VAR: &str = "RUST_LOG";
pub const PROJECT_LOG_FILTER_ENV_VAR: &str = "SQL_INTELLISCAN_LOG_LEVEL";
pub const ENVIRONMENT_DEFAULT_LOG_FILTERS: &[(AppEnvironment, &str)] = &[
    (AppEnvironment::Development, "debug"),
    (AppEnvironment::Test, "warn"),
    (AppEnvironment::Staging, "info"),
    (AppEnvironment::Production, "warn"),
];

static LOGGING_INITIALIZED: OnceLock<()> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggingStackDecision {
    pub emitter: &'static str,
    pub subscriber: &'static str,
    pub file_appender: Option<&'static str>,
    pub environment_variable: &'static str,
    pub project_environment_variable: &'static str,
    pub runtime_environment_variable: &'static str,
    pub environment_default_filters: &'static [(AppEnvironment, &'static str)],
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
        project_environment_variable: PROJECT_LOG_FILTER_ENV_VAR,
        runtime_environment_variable: APP_ENVIRONMENT_ENV_VAR,
        environment_default_filters: ENVIRONMENT_DEFAULT_LOG_FILTERS,
    }
}

pub fn default_log_filter_for_environment(environment: AppEnvironment) -> &'static str {
    ENVIRONMENT_DEFAULT_LOG_FILTERS
        .iter()
        .find_map(|(configured_environment, filter)| {
            (*configured_environment == environment).then_some(*filter)
        })
        .expect("every supported application environment must have a default log filter")
}

pub fn build_log_filter_from_env_value(
    configured_filter: Result<String, std::env::VarError>,
) -> EnvFilter {
    build_log_filter_from_env_value_for_environment(configured_filter, AppEnvironment::Development)
}

pub fn build_log_filter_from_env_value_for_environment(
    configured_filter: Result<String, std::env::VarError>,
    environment: AppEnvironment,
) -> EnvFilter {
    configured_filter
        .ok()
        .and_then(|filter| EnvFilter::try_new(filter).ok())
        .unwrap_or_else(|| EnvFilter::new(default_log_filter_for_environment(environment)))
}

pub fn build_log_filter(
    rust_log_filter: Result<String, std::env::VarError>,
    project_log_filter: Result<String, std::env::VarError>,
    environment: AppEnvironment,
) -> EnvFilter {
    let default_filter = default_log_filter_for_environment(environment);

    rust_log_filter
        .ok()
        .and_then(|filter| EnvFilter::try_new(filter).ok())
        .or_else(|| {
            project_log_filter
                .ok()
                .and_then(|filter| EnvFilter::try_new(filter).ok())
        })
        .unwrap_or_else(|| EnvFilter::new(default_filter))
}

pub fn init_logging() -> Result<(), LoggingInitError> {
    if LOGGING_INITIALIZED.get().is_some() {
        return Ok(());
    }

    let filter = build_log_filter(
        std::env::var(LOG_FILTER_ENV_VAR),
        std::env::var(PROJECT_LOG_FILTER_ENV_VAR),
        current_app_environment(),
    );

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
