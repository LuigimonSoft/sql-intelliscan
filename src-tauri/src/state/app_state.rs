use std::{future::Future, pin::Pin, sync::Arc};

use sql_intelliscan_services::{
    errors::ServiceResult,
    models::ConnectionTestResult,
    repository_wiring::{BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory},
    ConnectionService, GreetingService,
};

pub(crate) type AppGreetingService = GreetingService<BackendMetadataRepositoryAdapter>;
pub(crate) type AppConnectionService = ConnectionService<SqlServerConnectionRepositoryFactory>;

type ConnectionValidationFuture<'a> =
    Pin<Box<dyn Future<Output = ServiceResult<ConnectionTestResult>> + Send + 'a>>;

pub trait GreetingServicePort: Send + Sync {
    fn greet(&self, name: &str) -> String;
}

pub trait ConnectionServicePort: Send + Sync {
    fn test_connection(&self) -> ConnectionValidationFuture<'_>;
    fn test_connection_with_connection_string<'a>(
        &'a self,
        connection_string: &'a str,
    ) -> ConnectionValidationFuture<'a>;
}

impl GreetingServicePort for AppGreetingService {
    fn greet(&self, name: &str) -> String {
        GreetingService::greet(self, name)
    }
}

pub struct ConfiguredConnectionService {
    service: AppConnectionService,
    connection_string: Option<String>,
}

impl ConfiguredConnectionService {
    pub fn new(service: AppConnectionService, connection_string: Option<String>) -> Self {
        Self {
            service,
            connection_string,
        }
    }
}

impl ConnectionServicePort for ConfiguredConnectionService {
    fn test_connection(&self) -> ConnectionValidationFuture<'_> {
        Box::pin(async move {
            let connection_string = self.connection_string.as_deref().ok_or(
                sql_intelliscan_services::errors::ServiceError::InvalidConfiguration(
                    "SQL Server connection string is not configured",
                ),
            )?;

            self.service
                .test_configured_connection(connection_string)
                .await
        })
    }

    fn test_connection_with_connection_string<'a>(
        &'a self,
        connection_string: &'a str,
    ) -> ConnectionValidationFuture<'a> {
        Box::pin(async move {
            self.service
                .test_configured_connection(connection_string)
                .await
        })
    }
}

#[derive(Clone)]
pub struct AppState {
    greeting_service: Arc<dyn GreetingServicePort>,
    connection_service: Arc<dyn ConnectionServicePort>,
}

impl AppState {
    pub fn new(
        greeting_service: Arc<dyn GreetingServicePort>,
        connection_service: Arc<dyn ConnectionServicePort>,
    ) -> Self {
        Self {
            greeting_service,
            connection_service,
        }
    }

    pub fn greet(&self, name: &str) -> String {
        self.greeting_service.greet(name)
    }

    pub async fn test_connection(&self) -> ServiceResult<ConnectionTestResult> {
        self.connection_service.test_connection().await
    }

    pub async fn test_connection_with_connection_string(
        &self,
        connection_string: &str,
    ) -> ServiceResult<ConnectionTestResult> {
        self.connection_service
            .test_connection_with_connection_string(connection_string)
            .await
    }
}

pub type AppStateResult<T = AppState> = ServiceResult<T>;
