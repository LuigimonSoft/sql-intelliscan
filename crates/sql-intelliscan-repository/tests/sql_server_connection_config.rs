#![allow(non_snake_case)]

use sql_intelliscan_repository::{RepositoryError, SqlServerConnectionConfig};

#[test]
fn GivenConnectionStringWithMalformedSegment_WhenParsed_ThenResult_ShouldReturnInvalidConfiguration()
{
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
fn GivenConnectionStringWithEmptyServer_WhenParsed_ThenResult_ShouldReturnMissingHost()
{
    let result = SqlServerConnectionConfig::from_connection_string(
        "Server=   ;User Id=sa;Password=secret;Database=master;",
    );

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration("missing host"))
    );
}
