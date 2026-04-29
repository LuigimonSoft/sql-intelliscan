#![allow(non_snake_case)]

use sql_intelliscan_lib::{
    build_app_state, greet_with_state, register_handlers,
    validate_sql_server_connection_with_state, ServiceError,
};

#[test]
fn GivenValidName_WhenGreetCommandHandlerIsCalled_ThenMessage_ShouldIncludeNameAndBackendOrigin() {
    let app_state = build_app_state().expect("app state should build");

    let result = greet_with_state(&app_state, "Ana");

    assert_eq!(result, "Hello, Ana! You've been greeted from Rust!");
}

#[test]
fn GivenBuilder_WhenHandlersAreRegistered_ThenPipeline_ShouldBeComposable() {
    let builder = tauri::Builder::default();

    let _builder = register_handlers(builder);
}

#[test]
fn GivenInvalidConnectionString_WhenValidateCommandHandlerIsCalled_ThenResult_ShouldReturnServiceError(
) {
    let app_state = build_app_state().expect("app state should build");

    let result = tauri::async_runtime::block_on(validate_sql_server_connection_with_state(
        &app_state,
        "Server=localhost;Database=master",
    ));

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(error, ServiceError::InvalidConfiguration("missing username"));
}
