use std::time::Instant;

use tracing::info;

use crate::contracts::BackendMetadataRepository;

const LOG_TARGET: &str = "sql_intelliscan::services::greeting";
const SERVICE_NAME: &str = "GreetingService";

#[derive(Debug, Clone, Copy)]
pub struct GreetingService<R> {
    repository: R,
}

impl<R> GreetingService<R>
where
    R: BackendMetadataRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn greet(&self, name: &str) -> String {
        let started_at = Instant::now();

        info!(
            target: LOG_TARGET,
            service = SERVICE_NAME,
            operation = "greet",
            "Service operation started"
        );

        let greeting = format!(
            "Hello, {}! You've been greeted from {}!",
            name,
            self.repository.origin()
        );

        info!(
            target: LOG_TARGET,
            service = SERVICE_NAME,
            operation = "greet",
            elapsed_ms = started_at.elapsed().as_millis(),
            "Service operation completed successfully"
        );

        greeting
    }
}
