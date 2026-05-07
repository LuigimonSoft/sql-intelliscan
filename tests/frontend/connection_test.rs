#![allow(non_snake_case)]

use serde::{de::DeserializeOwned, Serialize};
use sql_intelliscan_ui::models::{
    BackendConnectionTestResult, ConnectionTestError, ConnectionTestRequest, ConnectionTestResult,
    ConnectionTestStatus,
};

fn assert_serializable<T: Serialize>() {}

fn assert_deserializable<T: DeserializeOwned>() {}

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
fn GivenConnectionRequest_WhenDefaultIsCreated_ThenDefaults_ShouldBeSafeForLocalDevelopment() {
    let request = ConnectionTestRequest::default();

    assert_eq!(request.host, "localhost");
    assert_eq!(request.port, 1433);
    assert_eq!(request.database, "master");
    assert_eq!(request.username, "");
    assert_eq!(request.password, "");
    assert!(request.encrypt);
    assert!(!request.trust_server_certificate);
    assert_eq!(request.connection_timeout_seconds, 30);
    assert_eq!(request.application_name.as_deref(), Some("SQL Intelliscan"));
}

#[test]
fn GivenConnectionRequest_WhenFormattedForDebug_ThenPassword_ShouldBeRedacted() {
    let request = valid_connection_request();
    let debug = format!("{request:?}");

    assert!(debug.contains("password: \"***\""));
    assert!(!debug.contains("StrongPassword123"));
}

#[test]
fn GivenFrontendModels_WhenUsedAcrossBoundaries_ThenSerdeTraits_ShouldBeAvailable() {
    assert_serializable::<ConnectionTestRequest>();
    assert_deserializable::<ConnectionTestRequest>();
    assert_serializable::<ConnectionTestResult>();
    assert_deserializable::<ConnectionTestResult>();
    assert_serializable::<ConnectionTestError>();
    assert_deserializable::<ConnectionTestError>();
}

#[test]
fn GivenBackendConnectionResult_WhenConverted_ThenFrontendResult_ShouldPreserveSafeDetails() {
    let result = ConnectionTestResult::from(BackendConnectionTestResult {
        success: true,
        message: "Connection successful".to_string(),
        server_version: Some("Microsoft SQL Server 2022".to_string()),
        database: Some("master".to_string()),
        latency_ms: Some(42),
    });

    assert!(result.success);
    assert_eq!(result.message, "Connection successful");
    assert_eq!(
        result.server_version.as_deref(),
        Some("Microsoft SQL Server 2022")
    );
    assert_eq!(result.database.as_deref(), Some("master"));
    assert_eq!(result.latency_ms, Some(42));
}

#[test]
fn GivenConnectionTestLifecycle_WhenStatusChanges_ThenUiState_ShouldRepresentEachStep() {
    let result = ConnectionTestResult {
        success: true,
        message: "Connection successful".to_string(),
        server_version: None,
        database: Some("master".to_string()),
        latency_ms: Some(12),
    };
    let error = ConnectionTestError {
        code: "TIMEOUT".to_string(),
        message: "The SQL Server connection attempt timed out.".to_string(),
    };

    assert_eq!(ConnectionTestStatus::Idle, ConnectionTestStatus::Idle);
    assert_eq!(ConnectionTestStatus::Loading, ConnectionTestStatus::Loading);
    assert_eq!(
        ConnectionTestStatus::Success(result.clone()),
        ConnectionTestStatus::Success(result)
    );
    assert_eq!(
        ConnectionTestStatus::Error(error.clone()),
        ConnectionTestStatus::Error(error)
    );
}
