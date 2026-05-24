use std::sync::OnceLock;

use tracing_subscriber::{fmt, EnvFilter};

pub const DEFAULT_LOG_FILTER: &str = "info";
pub const LOG_FILTER_ENV_VAR: &str = "RUST_LOG";

static LOGGING_INIT_RESULT: OnceLock<Result<(), String>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggingStackDecision {
    pub emitter: &'static str,
    pub subscriber: &'static str,
    pub file_appender: Option<&'static str>,
    pub environment_variable: &'static str,
    pub default_filter: &'static str,
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

pub fn init_logging() -> Result<(), String> {
    LOGGING_INIT_RESULT
        .get_or_init(|| {
            let filter = build_log_filter_from_env_value(std::env::var(LOG_FILTER_ENV_VAR));

            fmt()
                .with_env_filter(filter)
                .with_target(true)
                .with_level(true)
                .with_thread_ids(false)
                .try_init()
                .map_err(|error| format!("failed to initialize backend logging: {error}"))
        })
        .clone()
}
