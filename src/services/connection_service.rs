use crate::services::tauri_client::{
    invoke_test_connection, BackendConnectionTestResult, CommandErrorResponse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionTestStatus {
    pub success: bool,
    pub message: String,
    pub server_version: Option<String>,
    pub database: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendServiceError {
    pub message: String,
}

pub async fn test_connection() -> Result<ConnectionTestStatus, FrontendServiceError> {
    let response = invoke_test_connection()
        .await
        .map_err(normalize_backend_error)?;

    Ok(map_connection_test_result(response))
}

pub fn map_connection_test_result(response: BackendConnectionTestResult) -> ConnectionTestStatus {
    ConnectionTestStatus {
        success: response.success,
        message: response.message,
        server_version: response.server_version,
        database: response.database,
        latency_ms: response.latency_ms,
    }
}

pub fn normalize_backend_error(error: CommandErrorResponse) -> FrontendServiceError {
    FrontendServiceError {
        message: normalize_error_message(&error.message),
    }
}

fn normalize_error_message(message: &str) -> String {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "The backend returned an unknown error.".to_string();
    }

    trimmed.to_string()
}
