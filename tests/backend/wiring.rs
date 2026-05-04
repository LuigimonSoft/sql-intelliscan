#![allow(non_snake_case)]

use sql_intelliscan_lib::{
    build_app_state_with_connection_config, load_connection_config_from_connection_string,
    ServiceError,
};

#[test]
fn GivenValidStartupConfig_WhenAppStateIsBuilt_ThenServices_ShouldBeResolved() {
    let config = load_connection_config_from_connection_string(Some(
        "Server=localhost,1433;Database=master;User Id=sa;Password=secret;TrustServerCertificate=true;Encrypt=false;Connection Timeout=1",
    ))
    .expect("valid config should load");

    let app_state =
        build_app_state_with_connection_config(config).expect("app state should build");

    assert_eq!(
        app_state.greet("Marta"),
        "Hello, Marta! You've been greeted from Rust!"
    );
}

#[test]
fn GivenInvalidStartupConfig_WhenAppStateIsBuilt_ThenStartup_ShouldFailSafely() {
    let mut config = load_connection_config_from_connection_string(Some(
        "Server=localhost,1433;Database=master;User Id=sa;Password=secret;TrustServerCertificate=true;Encrypt=false;Connection Timeout=1",
    ))
    .expect("valid config should load");
    config.password.clear();

    let error = match build_app_state_with_connection_config(config) {
        Ok(_) => panic!("invalid startup config should fail"),
        Err(error) => error,
    };

    assert_eq!(error, ServiceError::InvalidConfiguration("missing password"));
}

#[test]
fn GivenUnavailableStartupServer_WhenStartupConnectionIsValidated_ThenError_ShouldBeSafe() {
    let config = load_connection_config_from_connection_string(Some(
        "Server=127.0.0.1,1;Database=master;User Id=sa;Password=secret;TrustServerCertificate=true;Encrypt=false;Connection Timeout=1",
    ))
    .expect("valid config should load");
    let app_state =
        build_app_state_with_connection_config(config).expect("app state should build");

    let result =
        tauri::async_runtime::block_on(app_state.validate_startup_sql_server_connection());
    let error = result.expect_err("unavailable startup server should fail safely");

    assert!(
        matches!(
            error,
            ServiceError::SourceUnavailable | ServiceError::QueryExecutionFailed
        ),
        "unexpected error variant: {error:?}"
    );
}
