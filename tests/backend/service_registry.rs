#![allow(non_snake_case)]

use sql_intelliscan_lib::{greet_user, validate_sql_server_connection};

#[test]
fn GivenValidName_WhenGreetUserIsCalled_ThenMessage_ShouldIncludeNameAndBackendOrigin() {
    let result = greet_user("Lucía");

    assert_eq!(result, "Hello, Lucía! You've been greeted from Rust!");
}

#[test]
fn GivenInvalidConnectionString_WhenValidationIsRequested_ThenResult_ShouldMapConfigurationError() {
    let result = tauri::async_runtime::block_on(validate_sql_server_connection(
        "Server=localhost;Database=master",
    ));

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(format!("{error:?}"), "InvalidConfiguration(\"missing username\")");
}

#[test]
fn GivenUnavailableServer_WhenValidationIsRequested_ThenResult_ShouldMapSourceUnavailable() {
    let result = tauri::async_runtime::block_on(validate_sql_server_connection(
        "Server=127.0.0.1,1;Database=master;User Id=sa;Password=bad-password;TrustServerCertificate=true;Encrypt=false;Connection Timeout=1",
    ));

    let error = result.expect_err("expected connection failure error");
    let debug_error = format!("{error:?}");
    assert!(
        debug_error == "SourceUnavailable" || debug_error == "QueryExecutionFailed",
        "unexpected error variant: {debug_error}"
    );
}
