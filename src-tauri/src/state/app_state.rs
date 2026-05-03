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
    fn validate_sql_server_connection(
        &self,
        connection_string: &str,
    ) -> ConnectionValidationFuture<'_>;
}

impl GreetingServicePort for AppGreetingService {
    fn greet(&self, name: &str) -> String {
        GreetingService::greet(self, name)
    }
}

impl ConnectionServicePort for AppConnectionService {
    fn validate_sql_server_connection(
        &self,
        connection_string: &str,
    ) -> ConnectionValidationFuture<'_> {
        let connection_string = connection_string.to_string();
        Box::pin(async move { self.test_configured_connection(&connection_string).await })
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

    pub async fn validate_sql_server_connection(
        &self,
        connection_string: &str,
    ) -> ServiceResult<ConnectionTestResult> {
        self.connection_service
            .validate_sql_server_connection(connection_string)
            .await
    }
}

pub type AppStateResult<T = AppState> = ServiceResult<T>;
