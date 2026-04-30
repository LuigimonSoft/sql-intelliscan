#![allow(non_snake_case)]

use sql_intelliscan_lib::{
    build_app_state, greet_command, greet_with_state, register_handlers,
    validate_sql_server_connection_command, validate_sql_server_connection_with_state,
    CommandErrorResponse,
};
use tauri::Manager;

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
fn GivenInvalidConnectionString_WhenValidateCommandHandlerIsCalled_ThenResult_ShouldReturnFriendlyError(
) {
    let app_state = build_app_state().expect("app state should build");

    let result = tauri::async_runtime::block_on(validate_sql_server_connection_with_state(
        &app_state,
        "Server=localhost;Database=master",
    ));

    let error = result.expect_err("expected invalid configuration error");
    let mapped_error = CommandErrorResponse::from_service_error(error);

    assert_eq!(
        mapped_error.message,
        "The provided configuration is invalid: missing username."
    );
}

#[test]
fn GivenManagedState_WhenGreetCommandIsCalled_ThenResponse_ShouldWrapGreetingMessage() {
    let app_state = build_app_state().expect("app state should build");
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let result = greet_command(app.state(), "Ana");

    assert_eq!(result.message, "Greeting generated successfully");
    assert_eq!(result.data, "Hello, Ana! You've been greeted from Rust!");
}

#[test]
fn GivenManagedStateAndInvalidConnectionString_WhenValidateCommandIsCalled_ThenResponse_ShouldReturnFriendlyError(
) {
    let app_state = build_app_state().expect("app state should build");
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let result = tauri::async_runtime::block_on(validate_sql_server_connection_command(
        app.state(),
        "Server=localhost;Database=master".to_string(),
    ));

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(
        error.message,
        "The provided configuration is invalid: missing username."
    );
}
