#![allow(non_snake_case)]

use sql_intelliscan_repository::{BackendMetadataRepository, StaticBackendMetadataRepository};

#[test]
fn GivenStaticBackendMetadataRepository_WhenOriginIsRequested_ThenValue_ShouldComeFromCommonLayer()
{
    let repository = StaticBackendMetadataRepository;

    assert_eq!(repository.origin(), "Rust");
}
