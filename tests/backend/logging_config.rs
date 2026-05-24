#![allow(non_snake_case)]

use std::env::VarError;

use sql_intelliscan_lib::{
    build_log_filter_from_env_value, init_logging, logging_stack_decision, DEFAULT_LOG_FILTER,
    LOG_FILTER_ENV_VAR,
};

#[test]
fn GivenLoggingStack_WhenDecisionIsReviewed_ThenSelection_ShouldUseTracingWithoutFileAppender() {
    let decision = logging_stack_decision();

    assert_eq!(decision.emitter, "tracing");
    assert_eq!(decision.subscriber, "tracing-subscriber");
    assert_eq!(decision.file_appender, None);
    assert_eq!(decision.environment_variable, LOG_FILTER_ENV_VAR);
    assert_eq!(decision.default_filter, DEFAULT_LOG_FILTER);
}

#[test]
fn GivenNoLogEnvironment_WhenFilterIsBuilt_ThenDefaultLevel_ShouldBeInfo() {
    let filter = build_log_filter_from_env_value(Err(VarError::NotPresent));

    assert_eq!(filter.to_string(), DEFAULT_LOG_FILTER);
}

#[test]
fn GivenConfiguredLogEnvironment_WhenFilterIsBuilt_ThenConfiguredLevel_ShouldBeUsed() {
    let filter = build_log_filter_from_env_value(Ok("sql_intelliscan=debug,warn".to_string()));

    assert_eq!(filter.to_string(), "sql_intelliscan=debug,warn");
}

#[test]
fn GivenInvalidLogEnvironment_WhenFilterIsBuilt_ThenDefaultLevel_ShouldBeUsed() {
    let filter =
        build_log_filter_from_env_value(Ok("sql_intelliscan=definitely_invalid_level".to_string()));

    assert_eq!(filter.to_string(), DEFAULT_LOG_FILTER);
}

#[test]
fn GivenBackendStartup_WhenLoggingInitializesRepeatedly_ThenInitialization_ShouldBeIdempotent() {
    init_logging().expect("logging should initialize");
    init_logging().expect("logging initialization should be idempotent");
}
