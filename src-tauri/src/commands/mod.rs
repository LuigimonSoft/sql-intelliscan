mod connection_commands;
mod greeting_commands;
mod response_models;

use tauri::State;

pub use response_models::{CommandErrorResponse, CommandSuccessResponse};
use sql_intelliscan_services::models::ConnectionTestResult;

use crate::state::AppState;

#[tauri::command]
pub fn greet_command(state: State<'_, AppState>, name: &str) -> CommandSuccessResponse<String> {
    greeting_commands::greet_command(state, name)
}

#[tauri::command]
pub async fn validate_sql_server_connection_command(
    state: State<'_, AppState>,
    connection_string: String,
) -> Result<CommandSuccessResponse<ConnectionTestResult>, CommandErrorResponse> {
    connection_commands::validate_sql_server_connection_command(state, connection_string).await
}

pub fn greet_with_state(state: &AppState, name: &str) -> String {
    greeting_commands::greet_with_state(state, name)
}

pub async fn validate_sql_server_connection_with_state(
    state: &AppState,
    connection_string: &str,
) -> sql_intelliscan_services::errors::ServiceResult<ConnectionTestResult> {
    connection_commands::validate_sql_server_connection_with_state(state, connection_string).await
}

pub fn register_handlers(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        greet_command,
        validate_sql_server_connection_command
    ])
}
