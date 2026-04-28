#![allow(non_snake_case)]

use sql_intelliscan_repository::{
    ConnectionRepository, SqlServerConnectionConfig, SqlServerConnectionRepository,
};

#[test]
#[ignore = "Requires a reachable SQL Server instance configured with SQLSERVER_TEST_CONNECTION_STRING"]
fn GivenConfiguredSqlServer_WhenConnectionValidationRuns_ThenRepository_ShouldReturnTrue() {
    let connection_string = std::env::var("SQLSERVER_TEST_CONNECTION_STRING")
        .expect("SQLSERVER_TEST_CONNECTION_STRING must be provided");

    let config = SqlServerConnectionConfig::from_connection_string(&connection_string)
        .expect("valid SQL Server connection string");

    let repository = SqlServerConnectionRepository::new(config);
    let runtime = tokio::runtime::Runtime::new().expect("Tokio runtime should start");
    let result = runtime
        .block_on(repository.validate_connection())
        .expect("connection validation should succeed");

    assert!(result);
}
