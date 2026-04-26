#![allow(non_snake_case)]

use sql_intelliscan_repository::BackendMetadataRepository;
use sql_intelliscan_services::{greet, GreetingService};

struct TestBackendMetadataRepository;

impl BackendMetadataRepository for TestBackendMetadataRepository {
    fn origin(&self) -> &'static str {
        "TestBackend"
    }
}

#[test]
fn GivenRepositoryDouble_WhenGreetingIsRequested_ThenService_ShouldUseRepositoryOrigin() {
    let service = GreetingService::new(TestBackendMetadataRepository);

    assert_eq!(
        service.greet("Carlos"),
        "Hello, Carlos! You've been greeted from TestBackend!"
    );
}

#[test]
fn GivenDefaultServiceFacade_WhenGreetingIsRequested_ThenIt_ShouldResolveRepositoryInternally() {
    assert_eq!(
        greet("Carlos"),
        "Hello, Carlos! You've been greeted from Rust!"
    );
}
