use crate::{state::AppState, CommandSuccessResponse};

pub fn greet_command(
    state: tauri::State<'_, AppState>,
    name: &str,
) -> CommandSuccessResponse<String> {
    let message = greet_with_state(state.inner(), name);

    CommandSuccessResponse {
        message: "Greeting generated successfully".to_string(),
        data: message,
    }
}

pub fn greet_with_state(state: &AppState, name: &str) -> String {
    state.greet(name)
}
