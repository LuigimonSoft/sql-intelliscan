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
fn GivenEmptyName_WhenInvokeGreetSyncIsCalled_ThenResponse_ShouldPreserveTemplate() {
    let response = invoke_greet_sync("");

    assert!(response.ok);
    assert_eq!(response.message, "Hello, ! You've been greeted from Rust!");
}

#[test]
fn GivenNativeAppComponent_WhenItIsBuilt_ThenItShouldCompileAsView() {
    let _view = view! { <App /> };
}
