#![allow(non_snake_case)]

use std::{
    fmt,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use sql_intelliscan_lib::{
    build_app_state, connection_string_from_request, greet_command, greet_with_state,
    register_handlers, test_connection, test_connection_with_state, AppState,
    CommandErrorResponse, ConnectionServicePort, ConnectionTestRequest, ConnectionTestResponse,
    GreetingServicePort, ServiceError,
};
use tauri::Manager;
use tracing::{
    field::{Field, Visit},
    Event, Subscriber,
};
use tracing_subscriber::{layer::Context, prelude::*, registry::LookupSpan, Layer};

#[derive(Clone)]
struct CapturedLogLayer {
    events: Arc<Mutex<Vec<String>>>,
}

#[derive(Default)]
struct CapturedLogVisitor {
    fields: Vec<String>,
}

impl Visit for CapturedLogVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields.push(format!("{}={value}", field.name()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields.push(format!("{}={value}", field.name()));
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        self.fields.push(format!("{}={value}", field.name()));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields.push(format!("{}={value}", field.name()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.fields.push(format!("{}={value:?}", field.name()));
    }
}

impl<S> Layer<S> for CapturedLogLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = CapturedLogVisitor::default();
        event.record(&mut visitor);

        let mut log_entry = format!("target={}", event.metadata().target());
        if !visitor.fields.is_empty() {
            log_entry.push(' ');
            log_entry.push_str(&visitor.fields.join(" "));
        }

        self.events
            .lock()
            .expect("captured log lock should not be poisoned")
            .push(log_entry);
    }
}

fn capture_logs<R>(operation: impl FnOnce() -> R) -> (R, Vec<String>) {
    let events = Arc::new(Mutex::new(Vec::new()));
    let subscriber = tracing_subscriber::registry().with(CapturedLogLayer {
        events: events.clone(),
    });

    let result = tracing::subscriber::with_default(subscriber, operation);
    let logs = events
        .lock()
        .expect("captured log lock should not be poisoned")
        .clone();

    (result, logs)
}

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
    }
}

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
    let mut request = valid_connection_request();
    request.username.clear();

    let result = tauri::async_runtime::block_on(test_connection_with_state(&app_state, request));

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
fn GivenName_WhenGreetCommandRuns_ThenLogs_ShouldIncludeSafeLifecycleMetadata() {
    let app_state = build_app_state().expect("app state should build");
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let (result, logs) = capture_logs(|| greet_command(app.state(), "Sensitive Name"));

    assert_eq!(result.message, "Greeting generated successfully");

    let logs = logs.join("\n");
    assert!(logs.contains("target=sql_intelliscan::commands"));
    assert!(logs.contains("command=greet_command"));
    assert!(logs.contains("message=Tauri command started"));
    assert!(logs.contains("message=Tauri command completed successfully"));
    assert!(logs.contains("elapsed_ms="));
    assert!(!logs.contains("Sensitive Name"));
}

#[test]
fn GivenManagedStateAndMissingConfiguration_WhenTestConnectionIsCalled_ThenResponse_ShouldReturnFriendlyError(
) {
    let app_state = build_app_state().expect("app state should build");
    let app = tauri::test::mock_builder()
        .manage(app_state)
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .expect("mock app should build");

    let mut request = valid_connection_request();
    request.password = "invalid;password".to_string();

    let result = tauri::async_runtime::block_on(test_connection(app.state(), request));

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

    fn test_connection_with_connection_string(
        &self,
        _connection_string: &str,
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

    let result = tauri::async_runtime::block_on(test_connection(app.state(), valid_connection_request()));

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

    fn test_connection_with_connection_string(
        &self,
        _connection_string: &str,
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

    let response: ConnectionTestResponse =
        tauri::async_runtime::block_on(test_connection(app.state(), valid_connection_request()))
            .expect("connection test should succeed");

    assert!(response.success);
    assert_eq!(response.message, "Connection successful");
    assert_eq!(response.database.as_deref(), Some("master"));
    assert_eq!(response.latency_ms, Some(42));
    assert_eq!(response.server_version, None);
}

#[test]
fn GivenConnectionRequest_WhenTestConnectionCommandSucceeds_ThenLogs_ShouldIncludeSafeLifecycleMetadata(
) {
    let app_state = AppState::new(
        Arc::new(MockGreetingService),
        Arc::new(SuccessfulConnectionService),
    );

    let (response, logs) = capture_logs(|| {
        tauri::async_runtime::block_on(test_connection_with_state(
            &app_state,
            valid_connection_request(),
        ))
    });

    response.expect("connection test should succeed");

    let logs = logs.join("\n");
    assert!(logs.contains("target=sql_intelliscan::commands"));
    assert!(logs.contains("command=test_connection"));
    assert!(logs.contains("message=Tauri command started"));
    assert!(logs.contains("message=Received connection test request metadata"));
    assert!(logs.contains("port=1433"));
    assert!(logs.contains("timeout_seconds=30"));
    assert!(logs.contains("encrypt=true"));
    assert!(logs.contains("trust_server_certificate=true"));
    assert!(logs.contains("message=Tauri command completed successfully"));
    assert!(logs.contains("elapsed_ms="));
    assert!(!logs.contains("StrongPassword123"));
    assert!(!logs.contains("Password="));
    assert!(!logs.contains("User Id="));
    assert!(!logs.contains("Server=localhost"));
    assert!(!logs.contains("username"));
}

#[test]
fn GivenConnectionRequestWithSecret_WhenConnectionCommandFails_ThenLogs_ShouldIncludeOnlySafeErrorCode(
) {
    let app_state = AppState::new(Arc::new(MockGreetingService), Arc::new(MockConnectionService));

    let (result, logs) = capture_logs(|| {
        tauri::async_runtime::block_on(test_connection_with_state(
            &app_state,
            valid_connection_request(),
        ))
    });

    let error = result.expect_err("expected service error");
    assert_eq!(error.code, "CONNECTION_FAILED");

    let logs = logs.join("\n");
    assert!(logs.contains("command=test_connection"));
    assert!(logs.contains("message=Tauri command failed"));
    assert!(logs.contains("error_code=CONNECTION_FAILED"));
    assert!(logs.contains("elapsed_ms="));
    assert!(!logs.contains("StrongPassword123"));
    assert!(!logs.contains("Password="));
    assert!(!logs.contains("User Id="));
    assert!(!logs.contains("Server=localhost"));
    assert!(!logs.contains("SourceUnavailable"));
}

#[derive(Default)]
struct RecordingConnectionService {
    connection_string: Mutex<Option<String>>,
}

impl ConnectionServicePort for RecordingConnectionService {
    fn test_connection(
        &self,
    ) -> BoxFuture<'_, Result<sql_intelliscan_lib::models::ConnectionTestResult, ServiceError>> {
        Box::pin(async { Err(ServiceError::SourceUnavailable) })
    }

    fn test_connection_with_connection_string(
        &self,
        connection_string: &str,
    ) -> BoxFuture<'_, Result<sql_intelliscan_lib::models::ConnectionTestResult, ServiceError>> {
        *self
            .connection_string
            .lock()
            .expect("connection string lock should not be poisoned") =
            Some(connection_string.to_string());

        Box::pin(async {
            Ok(sql_intelliscan_lib::models::ConnectionTestResult::valid_with_details(
                Some("inventory".to_string()),
                Some(7),
            ))
        })
    }
}

#[test]
fn GivenConnectionRequest_WhenTestConnectionCommandRuns_ThenCommand_ShouldUseRequestPayload() {
    let connection_service = Arc::new(RecordingConnectionService::default());
    let app_state = AppState::new(Arc::new(MockGreetingService), connection_service.clone());
    let mut request = valid_connection_request();
    request.database = "inventory".to_string();
    request.encrypt = false;

    let result = tauri::async_runtime::block_on(test_connection_with_state(&app_state, request))
        .expect("connection test should succeed");

    assert_eq!(result.database.as_deref(), Some("inventory"));
    assert_eq!(result.latency_ms, Some(7));
    assert_eq!(
        connection_service
            .connection_string
            .lock()
            .expect("connection string lock should not be poisoned")
            .as_deref(),
        Some(
            "Server=localhost,1433;Database=inventory;User Id=sa;Password=StrongPassword123;Encrypt=false;TrustServerCertificate=true;Connection Timeout=30"
        )
    );
}

#[test]
fn GivenConnectionRequestWithSemicolon_WhenConnectionStringIsBuilt_ThenCommand_ShouldRejectField() {
    let mut request = valid_connection_request();
    request.database = "master;Password=leaked".to_string();

    let error = connection_string_from_request(&request)
        .expect_err("semicolons in fields should be rejected");

    assert_eq!(
        error,
        ServiceError::InvalidConfiguration("connection fields must not contain semicolons")
    );
}

#[test]
fn GivenConnectionRequest_WhenFormattedForDebug_ThenPassword_ShouldBeRedacted() {
    let request = valid_connection_request();
    let debug = format!("{request:?}");

    assert!(debug.contains("password: \"***\""));
    assert!(!debug.contains("StrongPassword123"));
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
