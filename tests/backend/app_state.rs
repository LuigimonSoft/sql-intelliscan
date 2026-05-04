#![allow(non_snake_case)]

use std::{future::Future, pin::Pin, sync::Arc};

use sql_intelliscan_lib::{
    build_app_state, models::ConnectionTestResult, AppState, ConnectionServicePort,
    GreetingServicePort, ServiceError, StartupConnectionServicePort,
};

struct MockGreetingService;

impl GreetingServicePort for MockGreetingService {
    fn greet(&self, name: &str) -> String {
        format!("mock-{name}")
    }
}

struct MockConnectionService;

impl ConnectionServicePort for MockConnectionService {
    fn validate_sql_server_connection<'a>(
        &'a self,
        _connection_string: &'a str,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        sql_intelliscan_lib::models::ConnectionTestResult,
                        ServiceError,
                    >,
                > + Send
                + 'a,
        >,
    > {
        Box::pin(async { Ok(sql_intelliscan_lib::models::ConnectionTestResult::valid()) })
    }
}

struct MockStartupConnectionService {
    result: Result<ConnectionTestResult, ServiceError>,
}

impl StartupConnectionServicePort for MockStartupConnectionService {
    fn validate_startup_sql_server_connection(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<ConnectionTestResult, ServiceError>> + Send + '_>>
    {
        let result = self.result.clone();

        Box::pin(async move { result })
    }
}

#[test]
fn GivenDependencyWiring_WhenAppStateIsBuilt_ThenServices_ShouldBeResolved() {
    let app_state = build_app_state().expect("app state should build");

    assert_eq!(
        app_state.greet("Marta"),
        "Hello, Marta! You've been greeted from Rust!"
    );
}

#[test]
fn GivenInvalidConnectionString_WhenAppStateValidatesConnection_ThenError_ShouldBeClear() {
    let app_state = build_app_state().expect("app state should build");

    let result = tauri::async_runtime::block_on(
        app_state.validate_sql_server_connection("Server=localhost;Database=master"),
    );

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(error, ServiceError::InvalidConfiguration("missing username"));
}

#[test]
fn GivenStartupService_WhenStartupConnectionIsValidated_ThenResult_ShouldIncludeSafeDetails() {
    let app_state = AppState::with_startup_connection_service(
        Arc::new(MockGreetingService),
        Arc::new(MockConnectionService),
        Arc::new(MockStartupConnectionService {
            result: Ok(ConnectionTestResult::valid_with_details(
                Some("master".to_owned()),
                Some(7),
            )),
        }),
    );

    let result = tauri::async_runtime::block_on(
        app_state.validate_startup_sql_server_connection(),
    )
    .expect("startup validation should succeed");

    assert!(result.is_valid);
    assert_eq!(result.database, Some("master".to_owned()));
    assert_eq!(result.latency_ms, Some(7));
}

#[test]
fn GivenStateWithoutStartupService_WhenStartupConnectionIsValidated_ThenError_ShouldBeClear() {
    let app_state = AppState::new(Arc::new(MockGreetingService), Arc::new(MockConnectionService));

    let result =
        tauri::async_runtime::block_on(app_state.validate_startup_sql_server_connection());

    assert_eq!(
        result,
        Err(ServiceError::InvalidConfiguration(
            "startup connection service is not configured"
        ))
    );
}
