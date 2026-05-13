use crate::components::connection_test::ConnectionTestView;
#[cfg(target_arch = "wasm32")]
use crate::services::greeting_service::greet_message;
#[cfg(not(target_arch = "wasm32"))]
use crate::services::greeting_service::greet_message_sync;
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos::task::spawn_local;

#[cfg(target_arch = "wasm32")]
pub fn spawn_greet(name: String, set_greet_msg: WriteSignal<String>) {
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
    view! {
        <ConnectionTestView />
    }
}
