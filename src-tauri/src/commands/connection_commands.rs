use serde::Deserialize;
use sql_intelliscan_services::models::ConnectionTestResult;

use crate::{AppState, CommandErrorResponse, CommandSuccessResponse};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ValidateConnectionRequest {
    pub connection_string: String,
}

pub async fn validate_sql_server_connection_command(
    state: tauri::State<'_, AppState>,
    request: ValidateConnectionRequest,
) -> Result<CommandSuccessResponse<ConnectionTestResult>, CommandErrorResponse> {
    validate_sql_server_connection_with_state(state.inner(), &request.connection_string)
        .await
        .map(|result| CommandSuccessResponse {
            message: "Connection validated successfully".to_string(),
            data: result,
        })
        .map_err(CommandErrorResponse::from_service_error)
}

pub async fn validate_sql_server_connection_with_state(
    state: &AppState,
    connection_string: &str,
) -> sql_intelliscan_services::errors::ServiceResult<ConnectionTestResult> {
    state
        .validate_sql_server_connection(connection_string)
        .await
}
