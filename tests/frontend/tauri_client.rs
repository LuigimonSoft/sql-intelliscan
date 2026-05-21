#![allow(non_snake_case)]

use sql_intelliscan_ui::services::tauri_client::{
    invoke_backend_greet, invoke_test_connection, ConnectionTestRequest,
};

fn valid_connection_request() -> ConnectionTestRequest {
    ConnectionTestRequest {
        host: "localhost".to_string(),
        port: 1433,
        database: "master".to_string(),
        username: "sa".to_string(),
        password: "StrongPassword123".to_string(),
        encrypt: true,
        trust_server_certificate: true,
        connection_timeout_seconds: 30,
        application_name: Some("SQL Intelliscan Tests".to_string()),
    }
}

#[test]
fn GivenName_WhenGreetCommandIsInvoked_ThenMockedResponse_ShouldMatchBackendShape() {
    let response = futures::executor::block_on(invoke_backend_greet("Carlos"))
        .expect("native frontend test should use mocked Tauri response");

    assert_eq!(response.message, "Greeting generated successfully");
    assert_eq!(response.data, "Hello, Carlos! You've been greeted from Rust!");
}

#[test]
fn GivenPayload_WhenTestConnectionCommandIsInvoked_ThenMockedResponse_ShouldMatchBackendShape() {
    let response = futures::executor::block_on(invoke_test_connection(&valid_connection_request()))
        .expect("native frontend test should use mocked Tauri response");

    assert_eq!(response.message, "Connection successful");
    assert!(response.success);
    assert_eq!(response.database.as_deref(), Some("master"));
}

#[test]
fn GivenTauriClientSource_WhenReviewed_ThenConnectionCommand_ShouldUseExpectedBoundary() {
    let source = include_str!("../../src/services/tauri_client.rs");

    assert!(source.contains("pub struct ConnectionTestArgs<'a>"));
    assert!(source.contains("pub request: &'a ConnectionTestRequest"));
    assert!(source.contains("invoke(\"test_connection\", args)"));
    assert!(!source.contains("connection_string"));
    assert!(!source.contains("connectionString"));
    assert!(!source.contains("println!"));
    assert!(!source.contains("dbg!"));
}
