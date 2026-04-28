#![allow(non_snake_case)]

use sql_intelliscan_repository::{RepositoryError, SqlServerConnectionConfig};

#[test]
fn GivenConnectionStringWithMalformedSegment_WhenParsed_ThenResult_ShouldReturnInvalidConfiguration(
) {
    let result = SqlServerConnectionConfig::from_connection_string(
        "Server=localhost;MalformedSegment;User Id=sa;Password=secret;Database=master;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration(
            "invalid connection string segment"
        ))
    );
}

#[test]
fn GivenConnectionStringWithEmptyServer_WhenParsed_ThenResult_ShouldReturnMissingHost() {
    let result = SqlServerConnectionConfig::from_connection_string(
        "Server=   ;User Id=sa;Password=secret;Database=master;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration("missing host"))
    );
}

#[test]
fn GivenConnectionStringWithEmptyUsername_WhenParsed_ThenResult_ShouldReturnMissingUsername() {
    let result = SqlServerConnectionConfig::from_connection_string(
        "Server=localhost;User Id=   ;Password=secret;Database=master;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration("missing username"))
    );
}

#[test]
fn GivenConnectionStringWithEmptyPassword_WhenParsed_ThenResult_ShouldReturnMissingPassword() {
    let result = SqlServerConnectionConfig::from_connection_string(
        "Server=localhost;User Id=sa;Password=   ;Database=master;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration("missing password"))
    );
}

#[test]
fn GivenConnectionStringWithEmptyDatabase_WhenParsed_ThenResult_ShouldReturnMissingDatabase() {
    let result = SqlServerConnectionConfig::from_connection_string(
        "Server=localhost;User Id=sa;Password=secret;Database=   ;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration("missing database"))
    );
}
