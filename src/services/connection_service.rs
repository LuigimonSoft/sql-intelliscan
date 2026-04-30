use crate::services::tauri_client::{
    invoke_validate_sql_server_connection, BackendConnectionTestResult, CommandErrorResponse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionTestStatus {
    pub is_valid: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendServiceError {
    pub message: String,
}

pub async fn test_connection(
    connection_string: &str,
) -> Result<ConnectionTestStatus, FrontendServiceError> {
    let connection_string = normalize_connection_string(connection_string)?;
    let response = invoke_validate_sql_server_connection(connection_string)
        .await
        .map_err(normalize_backend_error)?;

    Ok(map_connection_test_result(response.data))
}

pub fn map_connection_test_result(result: BackendConnectionTestResult) -> ConnectionTestStatus {
    ConnectionTestStatus {
        is_valid: result.is_valid,
        message: result.message,
    }
}

pub fn normalize_backend_error(error: CommandErrorResponse) -> FrontendServiceError {
    FrontendServiceError {
        message: normalize_error_message(&error.message),
    }
}

fn normalize_connection_string(connection_string: &str) -> Result<&str, FrontendServiceError> {
    let trimmed = connection_string.trim();

    if trimmed.is_empty() {
        return Err(FrontendServiceError {
            message: "Connection string is required.".to_string(),
        });
    }

    Ok(trimmed)
}

fn normalize_error_message(message: &str) -> String {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "The backend returned an unknown error.".to_string();
    }

    trimmed.to_string()
}
