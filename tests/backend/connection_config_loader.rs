#![allow(non_snake_case)]

use sql_intelliscan_lib::load_connection_config_from_connection_string;

#[test]
fn GivenNoConfiguredConnectionString_WhenConfigIsLoaded_ThenDevelopmentConfig_ShouldBeValid() {
    let config = load_connection_config_from_connection_string(None)
        .expect("development config should be valid");

    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 1433);
    assert_eq!(config.database, "master");
    assert_eq!(config.username, "sa");
    assert_eq!(config.password, "development-password");
    assert_eq!(config.connection_timeout_seconds, 1);
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
