#![allow(non_snake_case)]

use sql_intelliscan_ui::services::connection_service::{
    map_connection_test_result, normalize_backend_error, test_connection, ConnectionTestRequest,
};
use sql_intelliscan_ui::services::tauri_client::{BackendConnectionTestResult, CommandErrorResponse};

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
fn GivenBackendConnectionResult_WhenMapped_ThenFrontendModel_ShouldExposeFriendlyStatus() {
    let status = map_connection_test_result(BackendConnectionTestResult {
        success: true,
        message: "Connection successful".to_string(),
        server_version: Some("Microsoft SQL Server 2022".to_string()),
        database: Some("master".to_string()),
        latency_ms: Some(42),
    });

    assert!(status.success);
    assert_eq!(status.message, "Connection successful");
    assert_eq!(
        status.server_version.as_deref(),
        Some("Microsoft SQL Server 2022")
    );
    assert_eq!(status.database.as_deref(), Some("master"));
    assert_eq!(status.latency_ms, Some(42));
}

#[test]
fn GivenBackendErrorWithWhitespace_WhenNormalized_ThenServiceError_ShouldTrimMessage() {
    let error = normalize_backend_error(CommandErrorResponse {
        code: "CONNECTION_FAILED".to_string(),
        message: "  The provided configuration is invalid.  ".to_string(),
    });

    assert_eq!(error.code, "CONNECTION_FAILED");
    assert_eq!(error.message, "The provided configuration is invalid.");
}

#[test]
fn GivenBackendErrorWithoutCode_WhenNormalized_ThenServiceError_ShouldUseFallbackCode() {
    let error = normalize_backend_error(CommandErrorResponse {
        code: " \t\n ".to_string(),
        message: "The backend returned an unexpected response.".to_string(),
    });

    assert_eq!(error.code, "UNEXPECTED_ERROR");
    assert_eq!(error.message, "The backend returned an unexpected response.");
}

#[test]
fn GivenBackendErrorWithoutMessage_WhenNormalized_ThenServiceError_ShouldUseFallbackMessage() {
    let error = normalize_backend_error(CommandErrorResponse {
        code: "UNEXPECTED_ERROR".to_string(),
        message: " \t\n ".to_string(),
    });

    assert_eq!(error.code, "UNEXPECTED_ERROR");
    assert_eq!(error.message, "Unable to test the SQL Server connection.");
}

#[test]
fn GivenConnectionRequest_WhenFormattedForDebug_ThenPassword_ShouldBeRedacted() {
    let request = valid_connection_request();
    let debug = format!("{request:?}");

    assert!(debug.contains("password: \"***\""));
    assert!(!debug.contains("StrongPassword123"));
}

#[test]
fn GivenConnectionPayload_WhenConnectionIsTested_ThenService_ShouldUseTauriClient() {
    let status = futures::executor::block_on(test_connection(valid_connection_request()))
        .expect("native frontend test uses a mocked Tauri client");

    assert!(status.success);
    assert_eq!(status.message, "Connection successful");
}
