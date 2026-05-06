#![allow(non_snake_case)]

use std::{future::Future, pin::Pin, sync::Arc};

use sql_intelliscan_lib::{
    build_app_state, greet_command, greet_with_state, register_handlers, test_connection,
    test_connection_with_state, AppState, CommandErrorResponse, ConnectionServicePort,
    ConnectionTestResponse, GreetingServicePort, ServiceError,
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
fn GivenMissingConfiguration_WhenTestConnectionHandlerIsCalled_ThenResult_ShouldReturnFriendlyError(
) {
    let app_state = build_app_state().expect("app state should build");

    let result = tauri::async_runtime::block_on(test_connection_with_state(&app_state));

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(error.code, "INVALID_CONFIGURATION");
    assert_eq!(
        error.message,
        "The SQL Server connection configuration is invalid."
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
fn GivenManagedStateAndMissingConfiguration_WhenTestConnectionIsCalled_ThenResponse_ShouldReturnFriendlyError(
) {
    let app_state = build_app_state().expect("app state should build");
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let result = tauri::async_runtime::block_on(test_connection(app.state()));

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(error.code, "INVALID_CONFIGURATION");
    assert_eq!(
        error.message,
        "The SQL Server connection configuration is invalid."
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
    fn test_connection(
        &self,
    ) -> BoxFuture<'_, Result<sql_intelliscan_lib::models::ConnectionTestResult, ServiceError>> {
        Box::pin(async { Err(ServiceError::SourceUnavailable) })
    }
}

#[test]
fn GivenMockedServices_WhenTestConnectionCommandRuns_ThenCommand_ShouldDelegateAndMapError() {
    let app_state = AppState::new(Arc::new(MockGreetingService), Arc::new(MockConnectionService));
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let result = tauri::async_runtime::block_on(test_connection(app.state()));

    let error = result.expect_err("expected service error");
    assert_eq!(error.code, "CONNECTION_FAILED");
    assert_eq!(
        error.message,
        "Unable to connect to the SQL Server instance."
    );
}

struct SuccessfulConnectionService;
impl ConnectionServicePort for SuccessfulConnectionService {
    fn test_connection(
        &self,
    ) -> BoxFuture<'_, Result<sql_intelliscan_lib::models::ConnectionTestResult, ServiceError>> {
        Box::pin(async {
            Ok(sql_intelliscan_lib::models::ConnectionTestResult::valid_with_details(
                Some("master".to_string()),
                Some(42),
            ))
        })
    }
}

#[test]
fn GivenSuccessfulServiceResult_WhenTestConnectionCommandRuns_ThenResponse_ShouldContainOnlySafeFields(
) {
    let app_state = AppState::new(
        Arc::new(MockGreetingService),
        Arc::new(SuccessfulConnectionService),
    );
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let response: ConnectionTestResponse = tauri::async_runtime::block_on(test_connection(app.state()))
        .expect("connection test should succeed");

    assert!(response.success);
    assert_eq!(response.message, "Connection successful");
    assert_eq!(response.database.as_deref(), Some("master"));
    assert_eq!(response.latency_ms, Some(42));
    assert_eq!(response.server_version, None);
}

#[test]
fn GivenServiceError_WhenMappedToCommandError_ThenResponse_ShouldUseStableSafeCode() {
    let error = CommandErrorResponse::from_service_error(ServiceError::QueryExecutionFailed);

    assert_eq!(error.code, "CONNECTION_FAILED");
    assert_eq!(
        error.message,
        "Unable to connect to the SQL Server instance."
    );
}
