#![allow(non_snake_case)]

use sql_intelliscan_ui::services::connection_service::{
    map_connection_test_result, normalize_backend_error, test_connection,
};
use sql_intelliscan_ui::services::tauri_client::{
    BackendConnectionTestResult, CommandErrorResponse,
};

#[test]
fn GivenBackendConnectionResult_WhenMapped_ThenFrontendModel_ShouldExposeFriendlyStatus() {
    let status = map_connection_test_result(BackendConnectionTestResult {
        is_valid: true,
        message: "Connection validated successfully".to_string(),
    });

    assert!(status.is_valid);
    assert_eq!(status.message, "Connection validated successfully");
}

#[test]
fn GivenBackendErrorWithWhitespace_WhenNormalized_ThenServiceError_ShouldTrimMessage() {
    let error = normalize_backend_error(CommandErrorResponse {
        message: "  The provided configuration is invalid.  ".to_string(),
    });

    assert_eq!(error.message, "The provided configuration is invalid.");
}

#[test]
fn GivenBackendErrorWithoutMessage_WhenNormalized_ThenServiceError_ShouldUseFallbackMessage() {
    let error = normalize_backend_error(CommandErrorResponse {
        message: " \t\n ".to_string(),
    });

    assert_eq!(error.message, "The backend returned an unknown error.");
}

#[test]
fn GivenEmptyConnectionString_WhenConnectionIsTested_ThenService_ShouldReturnValidationError() {
    let error = futures::executor::block_on(test_connection("  "))
        .expect_err("empty connection string should be rejected by frontend service");

    assert_eq!(error.message, "Connection string is required.");
}

#[test]
fn GivenConnectionStringWithWhitespace_WhenConnectionIsTested_ThenService_ShouldUseTauriClient() {
    let status = futures::executor::block_on(test_connection("  Server=localhost  "))
        .expect("native frontend test uses a mocked Tauri client");

    assert!(status.is_valid);
    assert_eq!(status.message, "Connection validated successfully");
}
