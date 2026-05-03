#![allow(non_snake_case)]

use sql_intelliscan_ui::services::tauri_client::{
    invoke_backend_greet, invoke_validate_sql_server_connection, validate_connection_args,
};

#[test]
fn GivenName_WhenGreetCommandIsInvoked_ThenMockedResponse_ShouldMatchBackendShape() {
    let response = futures::executor::block_on(invoke_backend_greet("Carlos"))
        .expect("native frontend test should use mocked Tauri response");

    assert_eq!(response.message, "Greeting generated successfully");
    assert_eq!(response.data, "Hello, Carlos! You've been greeted from Rust!");
}

#[test]
fn GivenConnectionString_WhenValidateCommandIsInvoked_ThenMockedResponse_ShouldMatchBackendShape() {
    let response = futures::executor::block_on(invoke_validate_sql_server_connection(
        "Server=localhost;Database=master",
    ))
    .expect("native frontend test should use mocked Tauri response");

    assert_eq!(response.message, "Connection validated successfully");
    assert!(response.data.is_valid);
}

#[test]
fn GivenConnectionString_WhenValidateArgsAreBuilt_ThenCommand_ShouldNestRequestPayload() {
    let args = validate_connection_args("Server=localhost;Database=master");

    assert_eq!(
        args.request.connection_string,
        "Server=localhost;Database=master"
    );
}
