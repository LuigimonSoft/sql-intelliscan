#![allow(non_snake_case)]

use sql_intelliscan_lib::{build_app_state, ServiceError};

#[test]
fn GivenNoDatabaseCredentials_WhenAppStateIsBuilt_ThenServices_ShouldBeResolved() {
    let app_state = build_app_state().expect("app state should build without database credentials");

    assert_eq!(
        app_state.greet("Marta"),
        "Hello, Marta! You've been greeted from Rust!"
    );
}

#[test]
fn GivenNoConfiguredConnectionString_WhenConnectionIsValidated_ThenError_ShouldBeSafe() {
    let app_state = build_app_state().expect("app state should build without database credentials");

    let result = tauri::async_runtime::block_on(app_state.test_connection());

    let error = result.expect_err("invalid configured connection should fail safely");

    assert_eq!(
        error,
        ServiceError::InvalidConfiguration("SQL Server connection string is not configured")
    );
}
