#[cfg(target_arch = "wasm32")]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackendConnectionTestResult {
    pub is_valid: bool,
}

#[derive(Serialize)]
struct ValidateConnectionArgs<'a> {
    connection_string: &'a str,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
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
            message: "Tauri backend is not available.".to_string(),
        });
    }

    let args = serde_wasm_bindgen::to_value(args).map_err(|_| CommandErrorResponse {
        message: "The frontend could not prepare backend command arguments.".to_string(),
    })?;

    let response = invoke(command, args).await;

    serde_wasm_bindgen::from_value(response).map_err(|_| CommandErrorResponse {
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
pub async fn invoke_validate_sql_server_connection(
    connection_string: &str,
) -> Result<CommandSuccessResponse<BackendConnectionTestResult>, CommandErrorResponse> {
    invoke_command(
        "validate_sql_server_connection_command",
        &ValidateConnectionArgs { connection_string },
    )
    .await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke_backend_greet(
    name: &str,
) -> Result<CommandSuccessResponse<String>, CommandErrorResponse> {
    let _args = GreetArgs { name };

    Ok(mock_greet_response(name))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke_validate_sql_server_connection(
    connection_string: &str,
) -> Result<CommandSuccessResponse<BackendConnectionTestResult>, CommandErrorResponse> {
    let _args = ValidateConnectionArgs { connection_string };

    Ok(CommandSuccessResponse {
        message: "Connection validated successfully".to_string(),
        data: BackendConnectionTestResult { is_valid: true },
    })
}

fn mock_greet_response(name: &str) -> CommandSuccessResponse<String> {
    CommandSuccessResponse {
        message: "Greeting generated successfully".to_string(),
        data: format!("Hello, {}! You've been greeted from Rust!", name),
    }
}
