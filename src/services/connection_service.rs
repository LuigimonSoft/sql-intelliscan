use std::fmt;

use serde::Serialize;

use crate::services::tauri_client::{
    invoke_test_connection, BackendConnectionTestResult, CommandErrorResponse,
};

#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct ConnectionTestRequest {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub encrypt: bool,
    pub trust_server_certificate: bool,
    pub connection_timeout_seconds: u64,
}

impl fmt::Debug for ConnectionTestRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectionTestRequest")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("database", &self.database)
            .field("username", &self.username)
            .field("password", &"***")
            .field("encrypt", &self.encrypt)
            .field("trust_server_certificate", &self.trust_server_certificate)
            .field(
                "connection_timeout_seconds",
                &self.connection_timeout_seconds,
            )
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub message: String,
    pub server_version: Option<String>,
    pub database: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionTestError {
    pub code: String,
    pub message: String,
}

pub async fn test_connection(
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResult, ConnectionTestError> {
    let response = invoke_test_connection(&request)
        .await
        .map_err(normalize_backend_error)?;

    Ok(map_connection_test_result(response))
}

pub fn map_connection_test_result(response: BackendConnectionTestResult) -> ConnectionTestResult {
    ConnectionTestResult {
        success: response.success,
        message: response.message,
        server_version: response.server_version,
        database: response.database,
        latency_ms: response.latency_ms,
    }
}

pub fn normalize_backend_error(error: CommandErrorResponse) -> ConnectionTestError {
    ConnectionTestError {
        code: normalize_error_code(&error.code),
        message: normalize_error_message(&error.message),
    }
}

fn normalize_error_code(code: &str) -> String {
    let trimmed = code.trim();

    if trimmed.is_empty() {
        return "UNEXPECTED_ERROR".to_string();
    }

    trimmed.to_ascii_uppercase()
}

fn normalize_error_message(message: &str) -> String {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "Unable to test the SQL Server connection.".to_string();
    }

    trimmed.to_string()
}
