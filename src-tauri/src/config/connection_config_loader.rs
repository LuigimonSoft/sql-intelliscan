use std::env::VarError;

use sql_intelliscan_repository::{RepositoryError, SqlServerConnectionConfig};
use sql_intelliscan_services::errors::{ServiceError, ServiceResult};

pub const CONNECTION_STRING_ENV_VAR: &str = "SQL_INTELLISCAN_SQLSERVER_CONNECTION_STRING";

/// Loads SQL Server configuration from
/// `SQL_INTELLISCAN_SQLSERVER_CONNECTION_STRING`.
pub fn load_connection_config() -> ServiceResult<SqlServerConnectionConfig> {
    load_connection_config_from_env_value(std::env::var(CONNECTION_STRING_ENV_VAR))
}

pub fn load_connection_config_from_env_value(
    configured_connection_string: Result<String, VarError>,
) -> ServiceResult<SqlServerConnectionConfig> {
    match configured_connection_string {
        Ok(connection_string) => {
            load_connection_config_from_connection_string(Some(&connection_string))
        }
        Err(VarError::NotPresent) => Err(ServiceError::InvalidConfiguration(
            "SQL Server connection string is not configured",
        )),
        Err(VarError::NotUnicode(_)) => Err(ServiceError::InvalidConfiguration(
            "SQL Server connection string must be valid Unicode",
        )),
    }
}

pub fn load_connection_config_from_connection_string(
    connection_string: Option<&str>,
) -> ServiceResult<SqlServerConnectionConfig> {
    let connection_string = match connection_string {
        Some(value) if value.trim().is_empty() => {
            return Err(ServiceError::InvalidConfiguration(
                "SQL Server connection string must not be empty",
            ));
        }
        Some(value) => value,
        None => {
            return Err(ServiceError::InvalidConfiguration(
                "SQL Server connection string is not configured",
            ));
        }
    };

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
