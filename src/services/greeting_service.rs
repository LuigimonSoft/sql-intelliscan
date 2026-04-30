use serde::{Deserialize, Serialize};

use crate::services::tauri_client::{
    invoke_backend_greet, CommandErrorResponse, CommandSuccessResponse,
};

#[derive(Deserialize, Serialize)]
pub struct GreetResponse {
    pub ok: bool,
    pub message: String,
}

pub fn should_send_greet(name: &str) -> bool {
    normalized_name(name).is_some()
}

pub fn invoke_greet_sync(name: &str) -> GreetResponse {
    GreetResponse {
        ok: true,
        message: format!("Hello, {}! You've been greeted from Rust!", name),
    }
}

pub fn greet_message_sync(name: &str) -> Option<String> {
    let normalized_name = normalized_name(name)?;

    Some(invoke_greet_sync(normalized_name).message)
}

pub async fn greet_message(name: &str) -> Option<String> {
    let normalized_name = normalized_name(name)?;

    Some(map_greet_response(
        invoke_backend_greet(normalized_name).await,
    ))
}

pub fn map_greet_response(
    response: Result<CommandSuccessResponse<String>, CommandErrorResponse>,
) -> String {
    match response {
        Ok(response) => response.data,
        Err(error) => error.message,
    }
}

fn normalized_name(name: &str) -> Option<&str> {
    let trimmed_name = name.trim();

    if trimmed_name.is_empty() {
        return None;
    }

    Some(trimmed_name)
}
