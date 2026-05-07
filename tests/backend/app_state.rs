#![allow(non_snake_case)]

use std::{future::Future, pin::Pin, sync::Arc};

use sql_intelliscan_lib::{
    build_app_state, AppState, ConnectionServicePort, GreetingServicePort, ServiceError,
};

struct MockGreetingService;

impl GreetingServicePort for MockGreetingService {
    fn greet(&self, name: &str) -> String {
        format!("mock-{name}")
    }
}

struct MockConnectionService;

impl ConnectionServicePort for MockConnectionService {
    fn test_connection(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        sql_intelliscan_lib::models::ConnectionTestResult,
                        ServiceError,
                    >,
                > + Send
                + '_,
        >,
    > {
        Box::pin(async { Ok(sql_intelliscan_lib::models::ConnectionTestResult::valid()) })
    }

    fn test_connection_with_connection_string(
        &self,
        _connection_string: &str,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        sql_intelliscan_lib::models::ConnectionTestResult,
                        ServiceError,
                    >,
                > + Send
                + '_,
        >,
    > {
        Box::pin(async { Ok(sql_intelliscan_lib::models::ConnectionTestResult::valid()) })
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

    let result = tauri::async_runtime::block_on(app_state.test_connection());

    let error = result.expect_err("expected invalid configuration error");
    assert_eq!(
        error,
        ServiceError::InvalidConfiguration("SQL Server connection string is not configured")
    );
}

#[test]
fn GivenConfiguredConnectionService_WhenStateTestsConnection_ThenResult_ShouldIncludeSafeDetails() {
    let app_state = AppState::new(Arc::new(MockGreetingService), Arc::new(MockConnectionService));

    let result = tauri::async_runtime::block_on(app_state.test_connection())
        .expect("configured validation should succeed");

    assert!(result.is_valid);
}
