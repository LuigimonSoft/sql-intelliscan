use std::time::Instant;

use tracing::info;

use crate::{state::AppState, CommandSuccessResponse};

const COMMAND_LOG_TARGET: &str = "sql_intelliscan::commands";
const GREET_COMMAND: &str = "greet_command";

pub fn greet_command(
    state: tauri::State<'_, AppState>,
    name: &str,
) -> CommandSuccessResponse<String> {
    let started_at = Instant::now();

    info!(
        target: COMMAND_LOG_TARGET,
        command = GREET_COMMAND,
        "Tauri command started"
    );

    let message = greet_with_state(state.inner(), name);

    info!(
        target: COMMAND_LOG_TARGET,
        command = GREET_COMMAND,
        elapsed_ms = started_at.elapsed().as_millis(),
        "Tauri command completed successfully"
    );

    CommandSuccessResponse {
        message: "Greeting generated successfully".to_string(),
        data: message,
    }
}

pub fn greet_with_state(state: &AppState, name: &str) -> String {
    state.greet(name)
}
