use crate::services::greeting_service::GreetResponse;
use serde::Serialize;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
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
pub async fn invoke_backend_greet(name: &str) -> GreetResponse {
    let args = match serde_wasm_bindgen::to_value(&GreetArgs { name }) {
        Ok(args) => args,
        Err(_) => {
            return GreetResponse {
                ok: false,
                message: "The frontend could not prepare the greet arguments.".to_string(),
            };
        }
    };

    if !has_tauri_invoke() {
        return GreetResponse {
            ok: true,
            message: format!("Hello, {}! You've been greeted from Rust!", name),
        };
    }

    let message = invoke("greet_command", args)
        .await
        .as_string()
        .unwrap_or_else(|| "The backend returned a non-string greeting.".to_string());

    GreetResponse { ok: true, message }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke_backend_greet(name: &str) -> GreetResponse {
    let _args = GreetArgs { name };

    GreetResponse {
        ok: true,
        message: format!("Hello, {}! You've been greeted from Rust!", name),
    }
}
