#![allow(non_snake_case)]

use sql_intelliscan_lib::{
    greet_command, register_handlers, validate_sql_server_connection_command,
};
use sql_intelliscan_services::errors::ServiceError;

#[test]
fn GivenValidName_WhenGreetCommandIsCalled_ThenMessage_ShouldIncludeNameAndBackendOrigin() {
    let result = greet_command("Ana");

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

    assert_eq!(result, Err(ServiceError::InvalidConfiguration("missing username")));
}
