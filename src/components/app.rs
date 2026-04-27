#[cfg(target_arch = "wasm32")]
use crate::services::greeting_service::greet_message;
#[cfg(not(target_arch = "wasm32"))]
use crate::services::greeting_service::greet_message_sync;
#[cfg(target_arch = "wasm32")]
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};

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
