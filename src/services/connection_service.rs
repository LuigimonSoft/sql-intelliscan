use crate::models::{BackendConnectionTestResult, ConnectionTestError, ConnectionTestResult};
use crate::services::tauri_client::{invoke_test_connection, CommandErrorResponse};

pub use crate::models::ConnectionTestRequest;

pub async fn test_connection(
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResult, ConnectionTestError> {
    let response = invoke_test_connection(&request)
        .await
        .map_err(normalize_backend_error)?;

    Ok(map_connection_test_result(response))
}

pub fn map_connection_test_result(response: BackendConnectionTestResult) -> ConnectionTestResult {
    response.into()
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
