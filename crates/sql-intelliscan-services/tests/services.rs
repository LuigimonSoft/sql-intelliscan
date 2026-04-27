#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use sql_intelliscan_services::{
    contracts::{
        AuditRepository, BackendMetadataRepository, ConfigurationRepository, ConnectionRepository,
    },
    AuditService, ConfigurationService, ConnectionService, GreetingService,
};

struct TestBackendMetadataRepository;

impl BackendMetadataRepository for TestBackendMetadataRepository {
    fn origin(&self) -> &'static str {
        "TestBackend"
    }
}

struct TestConnectionRepository;

impl ConnectionRepository for TestConnectionRepository {
    fn can_connect(&self, connection_id: &str) -> bool {
        connection_id == "valid-connection"
    }
}

#[derive(Clone)]
struct TestAuditRepository {
    entries: Rc<RefCell<Vec<String>>>,
}

impl AuditRepository for TestAuditRepository {
    fn save_entry(&self, entry: &str) {
        self.entries.borrow_mut().push(entry.to_owned());
    }
}

struct TestConfigurationRepository;

impl ConfigurationRepository for TestConfigurationRepository {
    fn find_value(&self, key: &str) -> Option<String> {
        match key {
            "theme" => Some("dark".to_owned()),
            _ => None,
        }
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
fn GivenConnectionRepository_WhenValidationIsRequested_ThenService_ShouldDelegateConnectivityCheck()
{
    let service = ConnectionService::new(TestConnectionRepository);

    assert!(service.validate_connection("valid-connection"));
    assert!(!service.validate_connection("invalid-connection"));
}

#[test]
fn GivenAuditRepository_WhenAuditEntryIsRegistered_ThenService_ShouldPersistEntryThroughRepository()
{
    let entries = Rc::new(RefCell::new(Vec::new()));
    let repository = TestAuditRepository {
        entries: Rc::clone(&entries),
    };
    let service = AuditService::new(repository);

    service.register_audit_entry("User logged in");

    assert_eq!(entries.borrow().as_slice(), ["User logged in"]);
}

#[test]
fn GivenConfigurationRepository_WhenValueIsRequested_ThenService_ShouldReturnValueFromRepository() {
    let service = ConfigurationService::new(TestConfigurationRepository);

    assert_eq!(
        service.get_configuration_value("theme"),
        Some("dark".to_owned())
    );
    assert_eq!(service.get_configuration_value("timezone"), None);
}
