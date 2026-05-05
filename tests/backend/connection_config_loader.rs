#![allow(non_snake_case)]

use std::{env::VarError, ffi::OsString};

use sql_intelliscan_lib::{
    load_connection_config_from_connection_string, load_connection_config_from_env_value,
};

#[test]
fn GivenNoConfiguredConnectionString_WhenConfigIsLoaded_ThenError_ShouldFailFast() {
    let result = load_connection_config_from_connection_string(None);

    let error = result.expect_err("missing configured value should be rejected");

    assert_eq!(
        error,
        sql_intelliscan_lib::ServiceError::InvalidConfiguration(
            "SQL Server connection string is not configured"
        )
    );
}

#[test]
fn GivenEnvironmentConnectionString_WhenConfigIsLoaded_ThenConfig_ShouldUseProvidedValues() {
    let config = load_connection_config_from_connection_string(Some(
        "Server=db.example.test,1444;Database=inventory;User Id=app;Password=secret;TrustServerCertificate=true;Encrypt=false;Connection Timeout=5",
    ))
    .expect("provided config should be valid");

    assert_eq!(config.host, "db.example.test");
    assert_eq!(config.port, 1444);
    assert_eq!(config.database, "inventory");
    assert_eq!(config.username, "app");
    assert_eq!(config.password, "secret");
    assert_eq!(config.connection_timeout_seconds, 5);
}

#[test]
fn GivenInvalidConnectionString_WhenConfigIsLoaded_ThenError_ShouldBeSafe() {
    let result = load_connection_config_from_connection_string(Some(
        "Server=db.example.test;Database=inventory;Password=secret",
    ));

    let error = result.expect_err("missing username should be rejected");

    assert_eq!(
        error,
        sql_intelliscan_lib::ServiceError::InvalidConfiguration("missing username")
    );
    assert!(!format!("{error:?}").contains("secret"));
}

#[test]
fn GivenEmptyConnectionString_WhenConfigIsLoaded_ThenError_ShouldFailFast() {
    let result = load_connection_config_from_connection_string(Some("   "));

    let error = result.expect_err("empty configured value should be rejected");

    assert_eq!(
        error,
        sql_intelliscan_lib::ServiceError::InvalidConfiguration(
            "SQL Server connection string must not be empty"
        )
    );
}

#[test]
fn GivenMissingEnvironmentValue_WhenConfigIsLoaded_ThenError_ShouldFailFast() {
    let result = load_connection_config_from_env_value(Err(VarError::NotPresent));

    let error = result.expect_err("missing environment value should be rejected");

    assert_eq!(
        error,
        sql_intelliscan_lib::ServiceError::InvalidConfiguration(
            "SQL Server connection string is not configured"
        )
    );
}

#[test]
fn GivenNonUnicodeEnvironmentValue_WhenConfigIsLoaded_ThenError_ShouldFailFast() {
    let result = load_connection_config_from_env_value(Err(VarError::NotUnicode(
        OsString::from("malformed"),
    )));

    let error = result.expect_err("malformed configured value should be rejected");

    assert_eq!(
        error,
        sql_intelliscan_lib::ServiceError::InvalidConfiguration(
            "SQL Server connection string must be valid Unicode"
        )
    );
}
