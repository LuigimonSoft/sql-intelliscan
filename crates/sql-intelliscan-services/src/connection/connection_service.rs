use std::time::Instant;

use tracing::{info, warn};

use crate::{
    contracts::{ConnectionRepository, ConnectionRepositoryFactory},
    errors::{ServiceError, ServiceResult},
    models::ConnectionTestResult,
};

const LOG_TARGET: &str = "sql_intelliscan::services::connection";
const SERVICE_NAME: &str = "ConnectionService";

#[derive(Debug, Clone)]
pub struct ConnectionService<R> {
    repository: R,
}

impl<R> ConnectionService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ConnectionService<R>
where
    R: ConnectionRepository,
{
    pub async fn test_connection(&self) -> ServiceResult<ConnectionTestResult> {
        let started_at = Instant::now();

        log_started("test_connection");

        match self.repository.validate_connection().await {
            Ok(result) => {
                log_succeeded("test_connection", started_at);
                Ok(result)
            }
            Err(error) => {
                let service_error = ServiceError::from(error);
                log_failed("test_connection", started_at, &service_error);
                Err(service_error)
            }
        }
    }

    pub async fn validate_connection(&self) -> ServiceResult<bool> {
        let started_at = Instant::now();

        log_started("validate_connection");

        match self.test_connection().await {
            Ok(result) => {
                log_succeeded("validate_connection", started_at);
                Ok(result.is_valid)
            }
            Err(error) => {
                log_failed("validate_connection", started_at, &error);
                Err(error)
            }
        }
    }
}

impl<F> ConnectionService<F>
where
    F: ConnectionRepositoryFactory,
{
    pub async fn test_configured_connection(
        &self,
        connection_string: &str,
    ) -> ServiceResult<ConnectionTestResult> {
        let started_at = Instant::now();

        log_started("test_configured_connection");

        let repository = match self.repository.build(connection_string) {
            Ok(repository) => repository,
            Err(error) => {
                let service_error = ServiceError::from(error);
                log_failed("test_configured_connection", started_at, &service_error);
                return Err(service_error);
            }
        };

        match repository.validate_connection().await {
            Ok(result) => {
                log_succeeded("test_configured_connection", started_at);
                Ok(result)
            }
            Err(error) => {
                let service_error = ServiceError::from(error);
                log_failed("test_configured_connection", started_at, &service_error);
                Err(service_error)
            }
        }
    }

    pub async fn validate_configured_connection(
        &self,
        connection_string: &str,
    ) -> ServiceResult<bool> {
        let started_at = Instant::now();

        log_started("validate_configured_connection");

        match self.test_configured_connection(connection_string).await {
            Ok(result) => {
                log_succeeded("validate_configured_connection", started_at);
                Ok(result.is_valid)
            }
            Err(error) => {
                log_failed("validate_configured_connection", started_at, &error);
                Err(error)
            }
        }
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
