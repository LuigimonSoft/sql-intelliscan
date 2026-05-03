#![allow(non_snake_case)]

use std::{future::Future, pin::Pin, sync::Arc};

use sql_intelliscan_lib::{
    build_app_state, greet_command, greet_with_state, register_handlers,
    validate_sql_server_connection_command, validate_sql_server_connection_with_state, AppState,
    CommandErrorResponse, ConnectionServicePort, GreetingServicePort, ServiceError,
    ValidateConnectionRequest,
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
        ValidateConnectionRequest {
            connection_string: "Server=localhost;Database=master".to_string(),
        },
    ));

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(
        error.message,
        "The provided configuration is invalid: missing username."
    );
}

struct MockGreetingService;
impl GreetingServicePort for MockGreetingService {
    fn greet(&self, name: &str) -> String {
        format!("mock-{name}")
    }
}

struct MockConnectionService;
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
impl ConnectionServicePort for MockConnectionService {
    fn validate_sql_server_connection<'a>(
        &'a self,
        _connection_string: &'a str,
    ) -> BoxFuture<'a, Result<sql_intelliscan_lib::models::ConnectionTestResult, ServiceError>> {
        Box::pin(async { Err(ServiceError::SourceUnavailable) })
    }
}

#[test]
fn GivenMockedServices_WhenValidateCommandRuns_ThenCommand_ShouldDelegateAndMapError() {
    let app_state = AppState::new(Arc::new(MockGreetingService), Arc::new(MockConnectionService));
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let result = tauri::async_runtime::block_on(validate_sql_server_connection_command(
        app.state(),
        ValidateConnectionRequest {
            connection_string: "ignored".to_string(),
        },
    ));

    let error = result.expect_err("expected service error");
    assert_eq!(error.message, "The data source is currently unavailable.");
}
