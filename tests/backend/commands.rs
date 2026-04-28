#![allow(non_snake_case)]

use sql_intelliscan_lib::{greet, register_handlers, validate_sql_server_connection_command};

#[test]
fn GivenValidName_WhenGreetFunctionIsCalled_ThenMessage_ShouldIncludeNameAndBackendOrigin() {
    let result = greet("Ana");

    assert_eq!(result, "Hello, Ana! You've been greeted from Rust!");
}

#[test]
fn GivenBuilder_WhenHandlersAreRegistered_ThenPipeline_ShouldBeComposable() {
    let builder = tauri::Builder::default();

    let _builder = register_handlers(builder);
}

#[test]
fn GivenInvalidConnectionString_WhenValidateCommandIsCalled_ThenResult_ShouldReturnServiceError() {
    let result = tauri::async_runtime::block_on(validate_sql_server_connection_command(
        "Server=localhost;Database=master".to_owned(),
    ));

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(format!("{error:?}"), "InvalidConfiguration(\"missing username\")");
}
