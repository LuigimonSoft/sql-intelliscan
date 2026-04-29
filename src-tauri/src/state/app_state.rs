use std::sync::Arc;

use sql_intelliscan_services::{
    errors::{ServiceError, ServiceResult},
    models::ConnectionTestResult,
    ConnectionService, GreetingService,
};

use crate::dependency_wiring::{
    BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory,
};

pub type AppGreetingService = GreetingService<BackendMetadataRepositoryAdapter>;
pub type AppConnectionService = ConnectionService<SqlServerConnectionRepositoryFactory>;

#[derive(Clone)]
pub struct AppState {
    greeting_service: Arc<AppGreetingService>,
    connection_service: Arc<AppConnectionService>,
}

impl AppState {
    pub fn new(
        greeting_service: Arc<AppGreetingService>,
        connection_service: Arc<AppConnectionService>,
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
    ) -> Result<ConnectionTestResult, ServiceError> {
        self.connection_service
            .test_configured_connection(connection_string)
            .await
    }

    pub fn greeting_service(&self) -> Arc<AppGreetingService> {
        Arc::clone(&self.greeting_service)
    }

    pub fn connection_service(&self) -> Arc<AppConnectionService> {
        Arc::clone(&self.connection_service)
    }
}

pub type AppStateResult<T = AppState> = ServiceResult<T>;
