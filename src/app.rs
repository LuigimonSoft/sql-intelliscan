use leptos::prelude::*;
use serde::{Deserialize, Serialize};

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

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Welcome to SQL Intelliscan"</h1>
            <p>"Initial version"</p>
            <p>"Native test renderer"</p>
        </main>
    }
}
