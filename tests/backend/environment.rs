#![allow(non_snake_case)]

use std::{env::VarError, ffi::OsString};

use sql_intelliscan_lib::{
    app_environment_from_env_value, parse_app_environment, AppEnvironment,
    APP_ENVIRONMENT_ENV_VAR,
};

#[test]
fn GivenSupportedEnvironmentNames_WhenParsed_ThenEnvironment_ShouldBeResolved() {
    assert_eq!(
        parse_app_environment("development"),
        Some(AppEnvironment::Development)
    );
    assert_eq!(parse_app_environment("TEST"), Some(AppEnvironment::Test));
    assert_eq!(parse_app_environment("stage"), Some(AppEnvironment::Staging));
    assert_eq!(
        parse_app_environment("prod"),
        Some(AppEnvironment::Production)
    );
}

#[test]
fn GivenMissingEnvironmentValue_WhenEnvironmentIsResolved_ThenDefault_ShouldBeDevelopment() {
    let environment = app_environment_from_env_value(Err(VarError::NotPresent));

    assert_eq!(environment, AppEnvironment::Development);
    assert_eq!(environment.as_str(), "development");
    assert_eq!(APP_ENVIRONMENT_ENV_VAR, "SQL_INTELLISCAN_ENV");
}

#[test]
fn GivenInvalidEnvironmentValue_WhenEnvironmentIsResolved_ThenFallback_ShouldBeProduction() {
    let environment = app_environment_from_env_value(Ok("unknown".to_owned()));

    assert_eq!(environment, AppEnvironment::Production);
}

#[test]
fn GivenNonUnicodeEnvironmentValue_WhenEnvironmentIsResolved_ThenFallback_ShouldBeProduction() {
    let environment =
        app_environment_from_env_value(Err(VarError::NotUnicode(OsString::from("invalid"))));

    assert_eq!(environment, AppEnvironment::Production);
}
