use sql_intelliscan_repository::{RepositoryError, SqlServerConnectionConfig};
use sql_intelliscan_services::errors::{ServiceError, ServiceResult};

pub const CONNECTION_STRING_ENV_VAR: &str = "SQL_INTELLISCAN_SQLSERVER_CONNECTION_STRING";

const DEVELOPMENT_CONNECTION_STRING: &str = "Server=localhost,1433;Database=master;User Id=sa;Password=development-password;TrustServerCertificate=true;Encrypt=false;Connection Timeout=1;Application Name=SQL Intelliscan;";

/// Loads SQL Server startup configuration from
/// `SQL_INTELLISCAN_SQLSERVER_CONNECTION_STRING`.
///
/// When the environment variable is absent, startup uses a local development
/// configuration so the application can compose services without production
/// credentials. Callers must not log the returned raw values.
pub fn load_connection_config() -> ServiceResult<SqlServerConnectionConfig> {
    let configured_connection_string = std::env::var(CONNECTION_STRING_ENV_VAR).ok();

    load_connection_config_from_connection_string(configured_connection_string.as_deref())
}

pub fn load_connection_config_from_connection_string(
    connection_string: Option<&str>,
) -> ServiceResult<SqlServerConnectionConfig> {
    let connection_string = connection_string
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(DEVELOPMENT_CONNECTION_STRING);

    SqlServerConnectionConfig::from_connection_string(connection_string)
        .map_err(map_repository_error_to_service)
}

fn map_repository_error_to_service(error: RepositoryError) -> ServiceError {
    match error {
        RepositoryError::InvalidConfiguration(reason) => ServiceError::InvalidConfiguration(reason),
        RepositoryError::SourceUnavailable => ServiceError::SourceUnavailable,
        RepositoryError::QueryExecutionFailed(_) => ServiceError::QueryExecutionFailed,
        RepositoryError::ResultMappingFailed(reason) => ServiceError::ResultMappingFailed(reason),
    }
}
