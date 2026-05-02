#![allow(non_snake_case)]

use sql_intelliscan_services::{
    contracts::{BackendMetadataRepository, ConnectionRepositoryFactory},
    errors::DataAccessError,
    repository_wiring::{BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory},
};

#[test]
fn GivenDefaultMetadataAdapter_WhenOriginIsRequested_ThenRepository_ShouldStayBehindServices() {
    let adapter = BackendMetadataRepositoryAdapter::default_static();

    assert_eq!(adapter.origin(), "Rust");
}

#[test]
fn GivenInvalidConnectionString_WhenRepositoryFactoryBuilds_ThenError_ShouldBeServiceDataAccessError(
) {
    let factory = SqlServerConnectionRepositoryFactory;

    let result = factory.build("Server=localhost;Database=master");

    assert_eq!(
        result.err(),
        Some(DataAccessError::InvalidConfiguration("missing username"))
    );
}
