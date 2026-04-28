use sql_intelliscan_repository::{
    BackendMetadataRepository as RepositoryBackendMetadataRepository,
    ConnectionRepository as RepositoryConnectionRepository, RepositoryError,
    SqlServerConnectionConfig, SqlServerConnectionRepository, StaticBackendMetadataRepository,
};
use sql_intelliscan_services::{
    contracts::{
        BackendMetadataRepository as ServiceBackendMetadataRepository,
        ConnectionRepository as ServiceConnectionRepository,
    },
    errors::{DataAccessError, DataAccessResult, ServiceError},
    models::ConnectionTestResult,
    ConnectionService, GreetingService,
};

pub fn greet_user(name: &str) -> String {
    let service = GreetingService::new(BackendMetadataRepositoryAdapter(
        StaticBackendMetadataRepository,
    ));

    service.greet(name)
}

pub async fn validate_sql_server_connection(
    connection_string: &str,
) -> Result<ConnectionTestResult, ServiceError> {
    let config = SqlServerConnectionConfig::from_connection_string(connection_string)
        .map_err(map_repository_error_to_data_access)
        .map_err(ServiceError::from)?;
    let repository =
        SqlServerConnectionRepositoryAdapter(SqlServerConnectionRepository::new(config));
    let service = ConnectionService::new(repository);

    service.test_connection().await
}

struct BackendMetadataRepositoryAdapter(StaticBackendMetadataRepository);

impl ServiceBackendMetadataRepository for BackendMetadataRepositoryAdapter {
    fn origin(&self) -> &'static str {
        self.0.origin()
    }
}

struct SqlServerConnectionRepositoryAdapter(SqlServerConnectionRepository);

impl ServiceConnectionRepository for SqlServerConnectionRepositoryAdapter {
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        self.0
            .validate_connection()
            .await
            .map_err(map_repository_error_to_data_access)
    }
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
