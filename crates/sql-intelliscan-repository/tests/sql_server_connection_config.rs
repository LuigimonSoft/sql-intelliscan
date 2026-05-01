#![allow(non_snake_case)]

use sql_intelliscan_repository::{
    ConnectionConfigValidationError, RepositoryError, SqlServerConnectionConfig,
};

#[test]
fn GivenValidConfiguration_WhenValidated_ThenResult_ShouldReturnOk() {
    let config = SqlServerConnectionConfig {
        host: "localhost".to_owned(),
        port: 1433,
        database: "master".to_owned(),
        username: "sa".to_owned(),
        password: "StrongPassword123".to_owned(),
        encrypt: true,
        trust_server_certificate: true,
        connection_timeout_seconds: 30,
        application_name: Some("SQL Intelliscan".to_owned()),
    };

    assert!(config.validate().is_ok());
}

#[test]
fn GivenMissingRequiredFields_WhenValidated_ThenResult_ShouldReturnAllErrors() {
    let config = SqlServerConnectionConfig {
        host: "   ".to_owned(),
        port: 1433,
        database: "".to_owned(),
        username: " ".to_owned(),
        password: "".to_owned(),
        encrypt: true,
        trust_server_certificate: false,
        connection_timeout_seconds: 30,
        application_name: Some("SQL Intelliscan".to_owned()),
    };

    let result = config.validate();

    assert!(result.is_err());
    let errors = result.expect_err("expected validation errors");
    assert!(errors.contains(&ConnectionConfigValidationError::HostRequired));
    assert!(errors.contains(&ConnectionConfigValidationError::DatabaseRequired));
    assert!(errors.contains(&ConnectionConfigValidationError::UsernameRequired));
    assert!(errors.contains(&ConnectionConfigValidationError::PasswordRequired));
}

#[test]
fn GivenInvalidPort_WhenValidated_ThenResult_ShouldReturnInvalidPortError() {
    let config = SqlServerConnectionConfig {
        port: 0,
        ..SqlServerConnectionConfig::default()
    };

    let errors = config.validate().expect_err("expected invalid port error");

    assert!(errors.contains(&ConnectionConfigValidationError::InvalidPort));
}

#[test]
fn GivenInvalidTimeoutValues_WhenValidated_ThenResult_ShouldReturnInvalidTimeoutError() {
    let zero_timeout_config = SqlServerConnectionConfig {
        connection_timeout_seconds: 0,
        ..SqlServerConnectionConfig::default()
    };

    let high_timeout_config = SqlServerConnectionConfig {
        connection_timeout_seconds: 301,
        ..SqlServerConnectionConfig::default()
    };

    assert!(zero_timeout_config
        .validate()
        .expect_err("expected timeout validation error")
        .contains(&ConnectionConfigValidationError::InvalidTimeout));

    assert!(high_timeout_config
        .validate()
        .expect_err("expected timeout validation error")
        .contains(&ConnectionConfigValidationError::InvalidTimeout));
}

#[test]
fn GivenSensitivePassword_WhenDebugFormatted_ThenOutput_ShouldMaskSecret() {
    let config = SqlServerConnectionConfig {
        username: "sa".to_owned(),
        password: "super-secret".to_owned(),
        ..SqlServerConnectionConfig::default()
    };

    let debug_output = format!("{:?}", config);

    assert!(!debug_output.contains("super-secret"));
    assert!(debug_output.contains("\"***\""));
}

#[test]
fn GivenConnectionStringWithoutServer_WhenParsed_ThenResult_ShouldReturnMissingHost() {
    let result = SqlServerConnectionConfig::from_connection_string(
        "User Id=sa;Password=secret;Database=master;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration("missing host"))
    );
}

#[test]
fn GivenConnectionStringWithoutDatabase_WhenParsed_ThenResult_ShouldReturnMissingDatabase() {
    let result = SqlServerConnectionConfig::from_connection_string(
        "Server=localhost;User Id=sa;Password=secret;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration("missing database"))
    );
}
