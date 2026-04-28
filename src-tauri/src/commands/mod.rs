use sql_intelliscan_services::{errors::ServiceError, models::ConnectionTestResult};

use crate::dependency_wiring::{greet_user, validate_sql_server_connection};

#[tauri::command]
pub fn greet_command(name: &str) -> String {
    greet_user(name)
}

#[tauri::command]
pub async fn validate_sql_server_connection_command(
    connection_string: String,
) -> Result<ConnectionTestResult, ServiceError> {
    validate_sql_server_connection(&connection_string).await
}

pub fn register_handlers(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        greet_command,
        validate_sql_server_connection_command
    ])
}
