#![allow(non_snake_case)]

use sql_intelliscan_ui::services::tauri_client::{
    invoke_backend_greet, invoke_test_connection,
};

#[test]
fn GivenName_WhenGreetCommandIsInvoked_ThenMockedResponse_ShouldMatchBackendShape() {
    let response = futures::executor::block_on(invoke_backend_greet("Carlos"))
        .expect("native frontend test should use mocked Tauri response");

    assert_eq!(response.message, "Greeting generated successfully");
    assert_eq!(response.data, "Hello, Carlos! You've been greeted from Rust!");
}

#[test]
fn GivenNoPayload_WhenTestConnectionCommandIsInvoked_ThenMockedResponse_ShouldMatchBackendShape() {
    let response = futures::executor::block_on(invoke_test_connection())
        .expect("native frontend test should use mocked Tauri response");

    assert_eq!(response.message, "Connection successful");
    assert!(response.success);
    assert_eq!(response.database.as_deref(), Some("master"));
}
