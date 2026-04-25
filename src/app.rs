#[cfg(target_arch = "wasm32")]
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

#[derive(Deserialize, Serialize)]
pub struct GreetResponse {
    pub ok: bool,
    pub message: String,
}

pub fn invoke_greet_sync(name: &str) -> GreetResponse {
    GreetResponse {
        ok: true,
        message: format!("Hello, {}! You've been greeted from Rust!", name),
    }
}

pub fn should_send_greet(name: &str) -> bool {
    !name.is_empty()
}

pub fn greet_message_sync(name: &str) -> Option<String> {
    if !should_send_greet(name) {
        return None;
    }

    Some(invoke_greet_sync(name).message)
}

#[derive(Serialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[cfg(target_arch = "wasm32")]
async fn invoke_greet(name: &str) -> GreetResponse {
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
        return invoke_greet_sync(name);
    }

    let message = invoke("greet_command", args)
        .await
        .as_string()
        .unwrap_or_else(|| "The backend returned a non-string greeting.".to_string());

    GreetResponse { ok: true, message }
}

#[cfg(not(target_arch = "wasm32"))]
async fn invoke_greet(name: &str) -> GreetResponse {
    let _args = GreetArgs { name };
    invoke_greet_sync(name)
}

pub async fn greet_message(name: &str) -> Option<String> {
    if !should_send_greet(name) {
        return None;
    }

    Some(invoke_greet(name).await.message)
}

#[cfg(target_arch = "wasm32")]
fn spawn_greet(name: String, set_greet_msg: WriteSignal<String>) {
    spawn_local(async move {
        if let Some(message) = greet_message(&name).await {
            set_greet_msg.set(message);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_greet(name: String, set_greet_msg: WriteSignal<String>) {
    if let Some(message) = greet_message_sync(&name) {
        set_greet_msg.set(message);
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| set_name.set(event_target_value(&ev));

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_greet(name.get_untracked(), set_greet_msg);
    };

    view! {
        <main class="container">
            <h1>"Welcome to SQL Intelliscan"</h1>
            <p>"Initial version"</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button id="greet-button" type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
        </main>
    }
}
