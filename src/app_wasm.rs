use js_sys::{Function, Object, Promise, Reflect};
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[derive(serde::Deserialize, serde::Serialize)]
struct GreetResponse {
    ok: bool,
    message: String,
}

async fn invoke_greet(name: &str) -> GreetResponse {
    let Some(window) = window() else {
        return GreetResponse {
            ok: false,
            message: "The browser window is not available.".to_string(),
        };
    };

    let window = JsValue::from(window);
    let tauri = Reflect::get(&window, &JsValue::from_str("__TAURI__")).ok();
    let core = tauri
        .as_ref()
        .and_then(|tauri| Reflect::get(tauri, &JsValue::from_str("core")).ok());
    let invoke = core
        .as_ref()
        .and_then(|core| Reflect::get(core, &JsValue::from_str("invoke")).ok())
        .and_then(|value| value.dyn_into::<Function>().ok());

    let Some(invoke) = invoke else {
        return GreetResponse {
            ok: true,
            message: format!("Hello, {}! You've been greeted from Rust!", name),
        };
    };

    let args = Object::new();
    if Reflect::set(&args, &JsValue::from_str("name"), &JsValue::from_str(name)).is_err() {
        return GreetResponse {
            ok: false,
            message: "The frontend could not prepare the greet arguments.".to_string(),
        };
    }

    let Some(core) = core else {
        return GreetResponse {
            ok: false,
            message: "The Tauri core API is not available.".to_string(),
        };
    };

    let result = match invoke.call2(&core, &JsValue::from_str("greet_command"), &args.into()) {
        Ok(value) => value,
        Err(error) => {
            return GreetResponse {
                ok: false,
                message: error
                    .as_string()
                    .unwrap_or_else(|| "The greet command failed.".to_string()),
            };
        }
    };

    let promise = match result.dyn_into::<Promise>() {
        Ok(promise) => promise,
        Err(_) => {
            return GreetResponse {
                ok: false,
                message: "The Tauri invoke call did not return a promise.".to_string(),
            };
        }
    };

    match JsFuture::from(promise).await {
        Ok(value) => GreetResponse {
            ok: true,
            message: value
                .as_string()
                .unwrap_or_else(|| "The backend returned a non-string greeting.".to_string()),
        },
        Err(error) => GreetResponse {
            ok: false,
            message: error
                .as_string()
                .unwrap_or_else(|| "The greet command failed.".to_string()),
        },
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                set_greet_msg.set("Please enter a name before greeting.".to_string());
                return;
            }

            set_is_loading.set(true);

            let response = invoke_greet(&name).await;

            set_greet_msg.set(response.message);
            set_is_loading.set(false);
        });
    };

    let current_name = move || name.get();
    let is_submitting = move || is_loading.get();
    let button_label = move || {
        if is_loading.get() {
            "Greeting..."
        } else {
            "Greet"
        }
    };
    let current_message = move || greet_msg.get();

    view! {
        <main class="container">
            <h1>"Welcome to SQL Intelliscan"</h1>
            <p>"Initial version"</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    prop:value=current_name
                    on:input=update_name
                />
                <button id="greet-button" type="submit" disabled=is_submitting>
                    {button_label}
                </button>
            </form>
            <p>{current_message}</p>
        </main>
    }
}
