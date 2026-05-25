use std::time::Instant;

use tracing::{info, warn};

use crate::{contracts::ConfigurationRepository, errors::ServiceError};

const LOG_TARGET: &str = "sql_intelliscan::services::configuration";
const SERVICE_NAME: &str = "ConfigurationService";

#[derive(Debug, Clone)]
pub struct ConfigurationService<R> {
    repository: R,
}

impl<R> ConfigurationService<R>
where
    R: ConfigurationRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn load_configuration_value(&self, key: &str) -> Result<Option<String>, ServiceError> {
        let started_at = Instant::now();

        log_started("load_configuration_value");

        let normalized_key = key.trim();
        if normalized_key.is_empty() {
            let error = ServiceError::InvalidConfiguration("configuration key is required");
            log_failed("load_configuration_value", started_at, &error);
            return Err(error);
        }

        let result = self.repository.find_value(normalized_key);
        log_succeeded("load_configuration_value", started_at);

        Ok(result)
    }

    pub fn get_configuration_value(&self, key: &str) -> Option<String> {
        let started_at = Instant::now();

        log_started("get_configuration_value");

        let result = self.repository.find_value(key);
        log_succeeded("get_configuration_value", started_at);

        result
    }
}

fn log_started(operation: &'static str) {
    info!(
        target: LOG_TARGET,
        service = SERVICE_NAME,
        operation = operation,
        "Service operation started"
    );
}

fn log_succeeded(operation: &'static str, started_at: Instant) {
    info!(
        target: LOG_TARGET,
        service = SERVICE_NAME,
        operation = operation,
        elapsed_ms = started_at.elapsed().as_millis(),
        "Service operation completed successfully"
    );
}

fn log_failed(operation: &'static str, started_at: Instant, error: &ServiceError) {
    warn!(
        target: LOG_TARGET,
        service = SERVICE_NAME,
        operation = operation,
        error_category = error.safe_category(),
        elapsed_ms = started_at.elapsed().as_millis(),
        "Service operation failed"
    );
}
