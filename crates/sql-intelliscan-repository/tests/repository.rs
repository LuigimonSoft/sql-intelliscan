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
        Err(RepositoryError::InvalidConfiguration("missing username"))
    );
}

#[test]
fn GivenRepositoryErrors_WhenSafeCategoryIsRequested_ThenResult_ShouldReturnDiagnosticCategories() {
    assert_eq!(
        RepositoryError::InvalidConfiguration("missing password").safe_category(),
        "INVALID_CONFIGURATION"
    );
    assert_eq!(
        RepositoryError::SourceUnavailable.safe_category(),
        "CONNECTION_FAILED"
    );
    assert_eq!(
        RepositoryError::QueryExecutionFailed("connection timeout".to_owned()).safe_category(),
        "TIMEOUT"
    );
    assert_eq!(
        RepositoryError::QueryExecutionFailed("validation failed".to_owned()).safe_category(),
        "QUERY_VALIDATION_FAILED"
    );
    assert_eq!(
        RepositoryError::ResultMappingFailed("unexpected scalar type").safe_category(),
        "QUERY_VALIDATION_FAILED"
    );
}
