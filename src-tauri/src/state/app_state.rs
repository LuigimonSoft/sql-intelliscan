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
    fn validate_sql_server_connection<'a>(
        &'a self,
        connection_string: &'a str,
    ) -> ConnectionValidationFuture<'a>;
}

pub trait StartupConnectionServicePort: Send + Sync {
    fn validate_startup_sql_server_connection(&self) -> ConnectionValidationFuture<'_>;
}

impl GreetingServicePort for AppGreetingService {
    fn greet(&self, name: &str) -> String {
        GreetingService::greet(self, name)
    }
}

impl ConnectionServicePort for AppConnectionService {
    fn validate_sql_server_connection<'a>(
        &'a self,
        connection_string: &'a str,
    ) -> ConnectionValidationFuture<'a> {
        Box::pin(self.test_configured_connection(connection_string))
    }
}

impl<R> StartupConnectionServicePort for ConnectionService<R>
where
    R: sql_intelliscan_services::contracts::ConnectionRepository + Send + Sync,
{
    fn validate_startup_sql_server_connection(&self) -> ConnectionValidationFuture<'_> {
        Box::pin(self.test_connection())
    }
}

struct DisabledStartupConnectionService;

impl StartupConnectionServicePort for DisabledStartupConnectionService {
    fn validate_startup_sql_server_connection(&self) -> ConnectionValidationFuture<'_> {
        Box::pin(async {
            Err(
                sql_intelliscan_services::errors::ServiceError::InvalidConfiguration(
                    "startup connection service is not configured",
                ),
            )
        })
    }
}

#[derive(Clone)]
pub struct AppState {
    greeting_service: Arc<dyn GreetingServicePort>,
    connection_service: Arc<dyn ConnectionServicePort>,
    startup_connection_service: Arc<dyn StartupConnectionServicePort>,
}

impl AppState {
    pub fn new(
        greeting_service: Arc<dyn GreetingServicePort>,
        connection_service: Arc<dyn ConnectionServicePort>,
    ) -> Self {
        Self {
            greeting_service,
            connection_service,
            startup_connection_service: Arc::new(DisabledStartupConnectionService),
        }
    }

    pub fn with_startup_connection_service(
        greeting_service: Arc<dyn GreetingServicePort>,
        connection_service: Arc<dyn ConnectionServicePort>,
        startup_connection_service: Arc<dyn StartupConnectionServicePort>,
    ) -> Self {
        Self {
            greeting_service,
            connection_service,
            startup_connection_service,
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

    pub async fn validate_startup_sql_server_connection(
        &self,
    ) -> ServiceResult<ConnectionTestResult> {
        self.startup_connection_service
            .validate_startup_sql_server_connection()
            .await
    }
}

pub type AppStateResult<T = AppState> = ServiceResult<T>;
