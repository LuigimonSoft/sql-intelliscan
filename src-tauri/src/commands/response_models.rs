use serde::Serialize;

use sql_intelliscan_services::errors::ServiceError;
use sql_intelliscan_services::models::ConnectionTestResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommandSuccessResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommandErrorResponse {
    pub code: String,
    pub message: String,
}

impl CommandErrorResponse {
    pub fn from_service_error(error: ServiceError) -> Self {
        let (code, message) = match error {
            ServiceError::InvalidAuditRequest(_) | ServiceError::InvalidConfiguration(_) => (
                "INVALID_CONFIGURATION",
                "The SQL Server connection configuration is invalid.",
            ),
            ServiceError::InvalidName => ("INVALID_CONFIGURATION", "The provided name is invalid."),
            ServiceError::QueryExecutionFailed => (
                "CONNECTION_FAILED",
                "Unable to connect to the SQL Server instance.",
            ),
            ServiceError::ResultMappingFailed(_) => (
                "UNEXPECTED_ERROR",
                "An unexpected error occurred while testing the connection.",
            ),
            ServiceError::SourceUnavailable => (
                "CONNECTION_FAILED",
                "Unable to connect to the SQL Server instance.",
            ),
        };

        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectionTestResponse {
    pub success: bool,
    pub message: String,
    pub server_version: Option<String>,
    pub database: Option<String>,
    pub latency_ms: Option<u64>,
}

impl From<ConnectionTestResult> for ConnectionTestResponse {
    fn from(result: ConnectionTestResult) -> Self {
        Self {
            success: result.is_valid,
            message: result.message,
            server_version: None,
            database: result.database,
            latency_ms: result.latency_ms,
        }
    }
}
