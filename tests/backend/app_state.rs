#![allow(non_snake_case)]

use sql_intelliscan_lib::build_app_state;

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
    assert_eq!(format!("{error:?}"), "InvalidConfiguration(\"missing username\")");
}
