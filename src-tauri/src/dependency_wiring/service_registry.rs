use std::sync::{Arc, OnceLock};

use sql_intelliscan_repository::{
    BackendMetadataRepository as RepositoryBackendMetadataRepository, RepositoryError,
    SqlServerConnectionConfig, SqlServerConnectionRepository, StaticBackendMetadataRepository,
};
use sql_intelliscan_services::{
    contracts::{
        BackendMetadataRepository as ServiceBackendMetadataRepository, ConnectionRepositoryFactory,
    },
    errors::{DataAccessError, DataAccessResult, ServiceError},
    ConnectionService, GreetingService,
};

use crate::state::{AppState, AppStateResult};

static SHARED_APP_STATE: OnceLock<AppStateResult> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub struct BackendMetadataRepositoryAdapter(pub StaticBackendMetadataRepository);

impl ServiceBackendMetadataRepository for BackendMetadataRepositoryAdapter {
    fn origin(&self) -> &'static str {
        self.0.origin()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SqlServerConnectionRepositoryFactory;

impl ConnectionRepositoryFactory for SqlServerConnectionRepositoryFactory {
    type Repository = SqlServerConnectionRepository;

    fn build(&self, connection_string: &str) -> DataAccessResult<Self::Repository> {
        let config = SqlServerConnectionConfig::from_connection_string(connection_string)
            .map_err(map_repository_error_to_data_access)?;

        Ok(SqlServerConnectionRepository::new(config))
    }
}

pub fn build_app_state() -> AppStateResult {
    let backend_metadata_repository =
        BackendMetadataRepositoryAdapter(StaticBackendMetadataRepository);
    let sql_server_connection_repository_factory = SqlServerConnectionRepositoryFactory;

    let greeting_service = Arc::new(GreetingService::new(backend_metadata_repository));
    let connection_service = Arc::new(ConnectionService::new(
        sql_server_connection_repository_factory,
    ));

    Ok(AppState::new(greeting_service, connection_service))
}

pub fn shared_app_state() -> AppStateResult {
    SHARED_APP_STATE.get_or_init(build_app_state).clone()
}

pub fn greet_user(name: &str) -> Result<String, ServiceError> {
    Ok(shared_app_state()?.greet(name))
}

pub async fn validate_sql_server_connection(
    connection_string: &str,
) -> Result<sql_intelliscan_services::models::ConnectionTestResult, ServiceError> {
    shared_app_state()?
        .validate_sql_server_connection(connection_string)
        .await
}

fn map_repository_error_to_data_access(error: RepositoryError) -> DataAccessError {
    match error {
        RepositoryError::SourceUnavailable => DataAccessError::SourceUnavailable,
        RepositoryError::InvalidConfiguration(reason) => {
            DataAccessError::InvalidConfiguration(reason)
        }
        RepositoryError::QueryExecutionFailed(reason) => {
            DataAccessError::QueryExecutionFailed(reason)
        }
        RepositoryError::ResultMappingFailed(reason) => {
            DataAccessError::ResultMappingFailed(reason)
        }
    }
}
