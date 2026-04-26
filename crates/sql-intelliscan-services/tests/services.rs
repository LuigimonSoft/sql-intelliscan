#![allow(non_snake_case)]

use sql_intelliscan_repository::BackendMetadataRepository;
use sql_intelliscan_services::GreetingService;

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
