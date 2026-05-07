#[cfg(target_arch = "wasm32")]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use crate::models::{BackendConnectionTestResult, ConnectionTestRequest};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CommandSuccessResponse<T> {
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CommandErrorResponse {
    pub code: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ConnectionTestArgs<'a> {
    pub request: &'a ConnectionTestRequest,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(
    inline_js = "export function hasTauriInvoke() { return !!(window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke); }"
)]
extern "C" {
    #[wasm_bindgen(js_name = hasTauriInvoke)]
    fn has_tauri_invoke() -> bool;
}

#[cfg(target_arch = "wasm32")]
async fn invoke_command<T, A>(
    command: &str,
    args: &A,
) -> Result<CommandSuccessResponse<T>, CommandErrorResponse>
where
    T: DeserializeOwned,
    A: Serialize,
{
    if !has_tauri_invoke() {
        return Err(CommandErrorResponse {
            code: "BACKEND_UNAVAILABLE".to_string(),
            message: "Tauri backend is not available.".to_string(),
        });
    }

    let args = serde_wasm_bindgen::to_value(args).map_err(|_| CommandErrorResponse {
        code: "INVALID_ARGUMENTS".to_string(),
        message: "The frontend could not prepare backend command arguments.".to_string(),
    })?;

    let response = invoke(command, args).await.map_err(map_invoke_error)?;

    serde_wasm_bindgen::from_value(response).map_err(|_| CommandErrorResponse {
        code: "UNEXPECTED_RESPONSE".to_string(),
        message: "The backend returned an unexpected response.".to_string(),
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn invoke_backend_greet(
    name: &str,
) -> Result<CommandSuccessResponse<String>, CommandErrorResponse> {
    if !has_tauri_invoke() {
        return Ok(mock_greet_response(name));
    }

    invoke_command("greet_command", &GreetArgs { name }).await
}

#[cfg(target_arch = "wasm32")]
pub async fn invoke_test_connection(
    request: &ConnectionTestRequest,
) -> Result<BackendConnectionTestResult, CommandErrorResponse> {
    if !has_tauri_invoke() {
        return Err(CommandErrorResponse {
            code: "BACKEND_UNAVAILABLE".to_string(),
            message: "Tauri backend is not available.".to_string(),
        });
    }

    let args = serde_wasm_bindgen::to_value(&ConnectionTestArgs { request }).map_err(|_| {
        CommandErrorResponse {
            code: "INVALID_ARGUMENTS".to_string(),
            message: "The frontend could not prepare backend command arguments.".to_string(),
        }
    })?;

    let response = invoke("test_connection", args)
        .await
        .map_err(map_invoke_error)?;

    serde_wasm_bindgen::from_value(response).map_err(|_| CommandErrorResponse {
        code: "UNEXPECTED_RESPONSE".to_string(),
        message: "The backend returned an unexpected response.".to_string(),
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke_backend_greet(
    name: &str,
) -> Result<CommandSuccessResponse<String>, CommandErrorResponse> {
    let _args = GreetArgs { name };

    Ok(mock_greet_response(name))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke_test_connection(
    request: &ConnectionTestRequest,
) -> Result<BackendConnectionTestResult, CommandErrorResponse> {
    let _args = ConnectionTestArgs { request };

    Ok(BackendConnectionTestResult {
        success: true,
        message: "Connection successful".to_string(),
        server_version: None,
        database: Some("master".to_string()),
        latency_ms: Some(1),
    })
}

fn mock_greet_response(name: &str) -> CommandSuccessResponse<String> {
    CommandSuccessResponse {
        message: "Greeting generated successfully".to_string(),
        data: format!("Hello, {}! You've been greeted from Rust!", name),
    }
}

#[cfg(target_arch = "wasm32")]
fn map_invoke_error(error: JsValue) -> CommandErrorResponse {
    serde_wasm_bindgen::from_value(error).unwrap_or_else(|_| CommandErrorResponse {
        code: "BACKEND_ERROR".to_string(),
        message: "The backend command failed.".to_string(),
    })
}
