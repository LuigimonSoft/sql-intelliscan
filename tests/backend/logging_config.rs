#![allow(non_snake_case)]

use std::env::VarError;

use sql_intelliscan_lib::{
    build_log_filter, build_log_filter_from_env_value, build_log_filter_from_env_value_for_environment,
    default_log_filter_for_environment, init_logging, logging_stack_decision, AppEnvironment,
    DEFAULT_LOG_FILTER, LOG_FILTER_ENV_VAR, PROJECT_LOG_FILTER_ENV_VAR,
};

#[test]
fn GivenLoggingStack_WhenDecisionIsReviewed_ThenSelection_ShouldUseTracingWithoutFileAppender() {
    let decision = logging_stack_decision();

    assert_eq!(decision.emitter, "tracing");
    assert_eq!(decision.subscriber, "tracing-subscriber");
    assert_eq!(decision.file_appender, None);
    assert_eq!(decision.environment_variable, LOG_FILTER_ENV_VAR);
    assert_eq!(decision.project_environment_variable, PROJECT_LOG_FILTER_ENV_VAR);
    assert_eq!(
        decision.runtime_environment_variable,
        sql_intelliscan_lib::APP_ENVIRONMENT_ENV_VAR
    );
    assert_eq!(
        decision.environment_default_filters,
        &[
            (AppEnvironment::Development, "debug"),
            (AppEnvironment::Test, "warn"),
            (AppEnvironment::Staging, "info"),
            (AppEnvironment::Production, "warn"),
        ]
    );
}

#[test]
fn GivenNoLogEnvironment_WhenFilterIsBuilt_ThenDefaultLevel_ShouldBeDevelopmentDebug() {
    let filter = build_log_filter_from_env_value(Err(VarError::NotPresent));

    assert_eq!(filter.to_string(), DEFAULT_LOG_FILTER);
}

#[test]
fn GivenConfiguredLogEnvironment_WhenFilterIsBuilt_ThenConfiguredLevel_ShouldBeUsed() {
    let filter = build_log_filter_from_env_value(Ok("sql_intelliscan=debug,warn".to_string()));

    assert_eq!(filter.to_string(), "sql_intelliscan=debug,warn");
}

#[test]
fn GivenInvalidLogEnvironment_WhenFilterIsBuilt_ThenEnvironmentDefault_ShouldBeUsed() {
    let filter = build_log_filter_from_env_value_for_environment(
        Ok("sql_intelliscan=definitely_invalid_level".to_string()),
        AppEnvironment::Production,
    );

    assert_eq!(filter.to_string(), "warn");
}

#[test]
fn GivenRuntimeEnvironments_WhenDefaultFilterIsRequested_ThenLevel_ShouldMatchEnvironmentPolicy() {
    assert_eq!(
        default_log_filter_for_environment(AppEnvironment::Development),
        "debug"
    );
    assert_eq!(default_log_filter_for_environment(AppEnvironment::Test), "warn");
    assert_eq!(
        default_log_filter_for_environment(AppEnvironment::Staging),
        "info"
    );
    assert_eq!(
        default_log_filter_for_environment(AppEnvironment::Production),
        "warn"
    );
}

#[test]
fn GivenRustLogOverride_WhenFilterIsBuilt_ThenRustLog_ShouldTakePrecedence() {
    let filter = build_log_filter(
        Ok("sql_intelliscan=trace,warn".to_owned()),
        Ok("info".to_owned()),
        AppEnvironment::Production,
    );

    assert_eq!(filter.to_string(), "sql_intelliscan=trace,warn");
}

#[test]
fn GivenProjectLogOverride_WhenRustLogIsMissing_ThenProjectLog_ShouldBeUsed() {
    let filter = build_log_filter(
        Err(VarError::NotPresent),
        Ok("sql_intelliscan_lib=info,warn".to_owned()),
        AppEnvironment::Production,
    );

    assert_eq!(filter.to_string(), "sql_intelliscan_lib=info,warn");
}

#[test]
fn GivenInvalidOverrides_WhenFilterIsBuilt_ThenEnvironmentDefault_ShouldBeUsed() {
    let filter = build_log_filter(
        Ok("bad=definitely_invalid".to_owned()),
        Ok("also=definitely_invalid".to_owned()),
        AppEnvironment::Test,
    );

    assert_eq!(filter.to_string(), "warn");
}

#[test]
fn GivenInvalidRustLogAndValidProjectOverride_WhenFilterIsBuilt_ThenProjectOverride_ShouldBeUsed() {
    let filter = build_log_filter(
        Ok("bad=definitely_invalid".to_owned()),
        Ok("info".to_owned()),
        AppEnvironment::Production,
    );

    assert_eq!(filter.to_string(), "info");
}

#[test]
fn GivenBackendStartup_WhenLoggingInitializesRepeatedly_ThenInitialization_ShouldBeIdempotent() {
    init_logging().expect("logging should initialize");
    init_logging().expect("logging initialization should be idempotent");
}
