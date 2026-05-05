#![allow(non_snake_case)]

use sql_intelliscan_lib::{greet_user, validate_sql_server_connection, ServiceError};

#[test]
fn GivenValidName_WhenGreetUserIsCalled_ThenMessage_ShouldIncludeNameAndBackendOrigin() {
    let result = greet_user("Lucía").expect("greeting should resolve");

    assert_eq!(result, "Hello, Lucía! You've been greeted from Rust!");
}

#[test]
fn GivenMissingConfiguration_WhenValidationIsRequested_ThenResult_ShouldMapConfigurationError() {
    let result = tauri::async_runtime::block_on(validate_sql_server_connection());

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(
        error,
        ServiceError::InvalidConfiguration("SQL Server connection string is not configured")
    );
}
