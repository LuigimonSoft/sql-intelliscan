mod connection_commands;
mod greeting_commands;
mod response_models;

use tauri::State;

pub use connection_commands::{connection_string_from_request, ConnectionTestRequest};
pub use response_models::{CommandErrorResponse, CommandSuccessResponse, ConnectionTestResponse};

use crate::state::AppState;

#[tauri::command]
pub fn greet_command(state: State<'_, AppState>, name: &str) -> CommandSuccessResponse<String> {
    greeting_commands::greet_command(state, name)
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResponse, CommandErrorResponse> {
    connection_commands::test_connection(state, request).await
}

pub fn greet_with_state(state: &AppState, name: &str) -> String {
    greeting_commands::greet_with_state(state, name)
}

pub async fn test_connection_with_state(
    state: &AppState,
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResponse, CommandErrorResponse> {
    connection_commands::test_connection_with_state(state, request).await
}

pub fn register_handlers(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![greet_command, test_connection])
}
