use sql_intelliscan_services::{errors::ServiceError, models::ConnectionTestResult};
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn greet_command(state: State<'_, AppState>, name: &str) -> String {
    greet_with_state(&state, name)
}

#[tauri::command]
pub async fn validate_sql_server_connection_command(
    state: State<'_, AppState>,
    connection_string: String,
) -> Result<ConnectionTestResult, ServiceError> {
    validate_sql_server_connection_with_state(&state, &connection_string).await
}

pub fn greet_with_state(state: &AppState, name: &str) -> String {
    state.greet(name)
}

pub async fn validate_sql_server_connection_with_state(
    state: &AppState,
    connection_string: &str,
) -> Result<ConnectionTestResult, ServiceError> {
    state
        .validate_sql_server_connection(connection_string)
        .await
}

pub fn register_handlers(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        greet_command,
        validate_sql_server_connection_command
    ])
}
