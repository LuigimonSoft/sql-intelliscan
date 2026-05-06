#![allow(non_snake_case)]

use std::{env::VarError, ffi::OsString};

use sql_intelliscan_lib::{
    build_app_state, configured_connection_string_from_env_value, ServiceError,
};

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

#[test]
fn GivenEmptyEnvironmentConnectionString_WhenWiringReadsConfig_ThenError_ShouldFailFast() {
    let result = configured_connection_string_from_env_value(Ok("   ".to_string()));

    let error = result.expect_err("empty configured value should fail before wiring state");

    assert_eq!(
        error,
        ServiceError::InvalidConfiguration("SQL Server connection string must not be empty")
    );
}

#[test]
fn GivenNonUnicodeEnvironmentConnectionString_WhenWiringReadsConfig_ThenError_ShouldFailFast() {
    let result = configured_connection_string_from_env_value(Err(VarError::NotUnicode(
        OsString::from("malformed"),
    )));

    let error = result.expect_err("non-unicode configured value should fail before wiring state");

    assert_eq!(
        error,
        ServiceError::InvalidConfiguration("SQL Server connection string must be valid Unicode")
    );
}
