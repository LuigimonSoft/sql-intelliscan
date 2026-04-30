#![allow(non_snake_case)]

use sql_intelliscan_repository::{
    BackendMetadataRepository, RepositoryError, SqlServerConnectionConfig,
    SqlServerMetadataRepository, StaticBackendMetadataRepository,
};

#[test]
fn GivenStaticBackendMetadataRepository_WhenOriginIsRequested_ThenValue_ShouldComeFromCommonLayer()
{
    let repository = StaticBackendMetadataRepository;

    assert_eq!(repository.origin(), "Rust");
}

#[test]
fn GivenSqlServerMetadataRepository_WhenOriginIsRequested_ThenValue_ShouldExposeSqlServerOrigin() {
    let repository = SqlServerMetadataRepository;

    assert_eq!(repository.origin(), "SQL Server");
}

#[test]
fn GivenConnectionStringWithoutCredentials_WhenParsed_ThenResult_ShouldReturnInvalidConfiguration()
{
    let result =
        SqlServerConnectionConfig::from_connection_string("Server=localhost;Database=master;");

    assert_eq!(
        result,
        Err(RepositoryError::InvalidConfiguration(
            "invalid SQL Server connection configuration"
        ))
    );
}
