use std::time::Instant;

use tracing::{info, warn};

use crate::{contracts::AuditRepository, errors::ServiceError};

const LOG_TARGET: &str = "sql_intelliscan::services::audit";
const SERVICE_NAME: &str = "AuditService";

#[derive(Debug, Clone)]
pub struct AuditService<R> {
    repository: R,
}

impl<R> AuditService<R>
where
    R: AuditRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn start_audit_execution(&self, request_name: &str) -> Result<(), ServiceError> {
        let started_at = Instant::now();

        log_started("start_audit_execution");

        let normalized_request_name = request_name.trim();
        if normalized_request_name.is_empty() {
            let error = ServiceError::InvalidAuditRequest("request name is required");
            log_failed("start_audit_execution", started_at, &error);
            return Err(error);
        }

        self.repository.save_entry(normalized_request_name);
        log_succeeded("start_audit_execution", started_at);

        Ok(())
    }

    pub fn register_audit_entry(&self, entry: &str) {
        let started_at = Instant::now();

        log_started("register_audit_entry");

        self.repository.save_entry(entry);
        log_succeeded("register_audit_entry", started_at);
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
