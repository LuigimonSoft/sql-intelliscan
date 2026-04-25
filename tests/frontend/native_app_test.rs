#![cfg(not(target_arch = "wasm32"))]
#![allow(non_snake_case)]

use leptos::prelude::*;
use sql_intelliscan_ui::app::{invoke_greet_sync, App};

#[test]
fn GivenName_WhenInvokeGreetSyncIsCalled_ThenResponse_ShouldContainGreeting() {
    let response = invoke_greet_sync("Carlos");

    assert!(response.ok);
    assert_eq!(response.message, "Hello, Carlos! You've been greeted from Rust!");
}

#[test]
fn GivenAppComponent_WhenRenderedToString_ThenMarkup_ShouldContainExpectedCopy() {
    let html = leptos::ssr::render_to_string(|| {
        view! { <App /> }
    });

    assert!(html.contains("Welcome to SQL Intelliscan"));
    assert!(html.contains("Initial version"));
    assert!(html.contains("greet-input"));
    assert!(html.contains("greet-button"));
}
