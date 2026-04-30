use sql_intelliscan_services::models::ConnectionTestResult;

use crate::{AppState, CommandErrorResponse, CommandSuccessResponse};

pub async fn validate_sql_server_connection_command(
    state: tauri::State<'_, AppState>,
    connection_string: String,
) -> Result<CommandSuccessResponse<ConnectionTestResult>, CommandErrorResponse> {
    validate_sql_server_connection_with_state(state.inner(), &connection_string)
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
