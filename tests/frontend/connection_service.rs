#![allow(non_snake_case)]

use sql_intelliscan_ui::services::connection_service::{
    map_connection_test_result, normalize_backend_error, test_connection,
};
use sql_intelliscan_ui::services::tauri_client::{BackendConnectionTestResult, CommandErrorResponse};

#[test]
fn GivenBackendConnectionResult_WhenMapped_ThenFrontendModel_ShouldExposeFriendlyStatus() {
    let status = map_connection_test_result(BackendConnectionTestResult {
        success: true,
        message: "Connection validated successfully".to_string(),
        server_version: None,
        database: Some("master".to_string()),
        latency_ms: Some(42),
    });

    assert!(status.success);
    assert_eq!(status.message, "Connection validated successfully");
    assert_eq!(status.database.as_deref(), Some("master"));
    assert_eq!(status.latency_ms, Some(42));
}

#[test]
fn GivenBackendErrorWithWhitespace_WhenNormalized_ThenServiceError_ShouldTrimMessage() {
    let error = normalize_backend_error(CommandErrorResponse {
        code: "CONNECTION_FAILED".to_string(),
        message: "  The provided configuration is invalid.  ".to_string(),
    });

    assert_eq!(error.message, "The provided configuration is invalid.");
}

#[test]
fn GivenBackendErrorWithoutMessage_WhenNormalized_ThenServiceError_ShouldUseFallbackMessage() {
    let error = normalize_backend_error(CommandErrorResponse {
        code: "UNEXPECTED_ERROR".to_string(),
        message: " \t\n ".to_string(),
    });

    assert_eq!(error.message, "The backend returned an unknown error.");
}

#[test]
fn GivenNoConnectionPayload_WhenConnectionIsTested_ThenService_ShouldUseTauriClient() {
    let status = futures::executor::block_on(test_connection())
        .expect("native frontend test uses a mocked Tauri client");

    assert!(status.success);
    assert_eq!(status.message, "Connection validated successfully");
}
