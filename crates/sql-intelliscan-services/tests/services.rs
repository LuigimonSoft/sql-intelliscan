#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use sql_intelliscan_services::{
    contracts::{
        AuditRepository, BackendMetadataRepository, ConfigurationRepository, ConnectionRepository,
        ConnectionRepositoryFactory,
    },
    errors::{DataAccessError, DataAccessResult, ServiceError},
    models::ConnectionTestResult,
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
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        Ok(true)
    }
}

struct FailingConnectionRepository;

impl ConnectionRepository for FailingConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        Err(DataAccessError::SourceUnavailable)
    }
}

struct QueryFailingConnectionRepository;

impl ConnectionRepository for QueryFailingConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        Err(DataAccessError::QueryExecutionFailed("timeout".to_owned()))
    }
}

struct MappingFailingConnectionRepository;

impl ConnectionRepository for MappingFailingConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        Err(DataAccessError::ResultMappingFailed(
            "unexpected scalar type",
        ))
    }
}

struct InvalidConfigurationConnectionRepository;

impl ConnectionRepository for InvalidConfigurationConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        Err(DataAccessError::InvalidConfiguration("missing host"))
    }
}

struct FalseConnectionRepository;

impl ConnectionRepository for FalseConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        Ok(false)
    }
}

struct TestConnectionRepositoryFactory;

impl ConnectionRepositoryFactory for TestConnectionRepositoryFactory {
    type Repository = TestConnectionRepository;

    fn build(&self, connection_string: &str) -> DataAccessResult<Self::Repository> {
        if connection_string.trim().is_empty() {
            return Err(DataAccessError::InvalidConfiguration(
                "connection string is required",
            ));
        }

        Ok(TestConnectionRepository)
    }
}

struct FalseConnectionRepositoryFactory;

impl ConnectionRepositoryFactory for FalseConnectionRepositoryFactory {
    type Repository = FalseConnectionRepository;

    fn build(&self, _connection_string: &str) -> DataAccessResult<Self::Repository> {
        Ok(FalseConnectionRepository)
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

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Ok(ConnectionTestResult::valid()));
}

#[test]
fn GivenConnectionRepository_WhenBooleanValidationIsRequested_ThenService_ShouldReturnBoolean() {
    let service = ConnectionService::new(TestConnectionRepository);

    let result = futures::executor::block_on(service.validate_connection());

    assert_eq!(result, Ok(true));
}

#[test]
fn GivenConnectionRepositoryReturnsFalse_WhenValidationIsRequested_ThenService_ShouldReturnInvalidResult(
) {
    let service = ConnectionService::new(FalseConnectionRepository);

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Ok(ConnectionTestResult::invalid()));
}

#[test]
fn GivenConnectionRepositoryFailure_WhenValidationIsRequested_ThenService_ShouldReturnServiceError()
{
    let service = ConnectionService::new(FailingConnectionRepository);

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Err(ServiceError::SourceUnavailable));
}

#[test]
fn GivenRepositoryQueryFailure_WhenValidationIsRequested_ThenService_ShouldNormalizeError() {
    let service = ConnectionService::new(QueryFailingConnectionRepository);

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Err(ServiceError::QueryExecutionFailed));
}

#[test]
fn GivenRepositoryMappingFailure_WhenValidationIsRequested_ThenService_ShouldNormalizeError() {
    let service = ConnectionService::new(MappingFailingConnectionRepository);

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(
        result,
        Err(ServiceError::ResultMappingFailed("unexpected scalar type"))
    );
}

#[test]
fn GivenRepositoryConfigurationFailure_WhenValidationIsRequested_ThenService_ShouldNormalizeError()
{
    let service = ConnectionService::new(InvalidConfigurationConnectionRepository);

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(
        result,
        Err(ServiceError::InvalidConfiguration("missing host"))
    );
}

#[test]
fn GivenConnectionRepositoryFactory_WhenConfiguredValidationIsRequested_ThenService_ShouldBuildRepository(
) {
    let service = ConnectionService::new(TestConnectionRepositoryFactory);

    let result =
        futures::executor::block_on(service.test_configured_connection("Server=localhost"));

    assert_eq!(result, Ok(ConnectionTestResult::valid()));
}

#[test]
fn GivenInvalidConnectionConfiguration_WhenConfiguredValidationIsRequested_ThenService_ShouldReturnValidationError(
) {
    let service = ConnectionService::new(TestConnectionRepositoryFactory);

    let result = futures::executor::block_on(service.test_configured_connection(" "));

    assert_eq!(
        result,
        Err(ServiceError::InvalidConfiguration(
            "connection string is required"
        ))
    );
}

#[test]
fn GivenConnectionRepositoryFactory_WhenConfiguredBooleanValidationIsRequested_ThenService_ShouldReturnBoolean(
) {
    let service = ConnectionService::new(FalseConnectionRepositoryFactory);

    let result =
        futures::executor::block_on(service.validate_configured_connection("Server=localhost"));

    assert_eq!(result, Ok(false));
}

#[test]
fn GivenAuditRepository_WhenAuditEntryIsRegistered_ThenService_ShouldPersistEntryThroughRepository()
{
    let entries = Rc::new(RefCell::new(Vec::new()));
    let repository = TestAuditRepository {
        entries: Rc::clone(&entries),
    };
    let service = AuditService::new(repository);

    let result = service.start_audit_execution(" User logged in ");

    assert_eq!(result, Ok(()));
    assert_eq!(entries.borrow().as_slice(), ["User logged in"]);
}

#[test]
fn GivenAuditRepository_WhenRawAuditEntryIsRegistered_ThenService_ShouldPersistEntryThroughRepository(
) {
    let entries = Rc::new(RefCell::new(Vec::new()));
    let repository = TestAuditRepository {
        entries: Rc::clone(&entries),
    };
    let service = AuditService::new(repository);

    service.register_audit_entry("User logged in");

    assert_eq!(entries.borrow().as_slice(), ["User logged in"]);
}

#[test]
fn GivenBlankAuditRequest_WhenAuditExecutionStarts_ThenService_ShouldReturnValidationError() {
    let entries = Rc::new(RefCell::new(Vec::new()));
    let repository = TestAuditRepository {
        entries: Rc::clone(&entries),
    };
    let service = AuditService::new(repository);

    let result = service.start_audit_execution(" ");

    assert_eq!(
        result,
        Err(ServiceError::InvalidAuditRequest(
            "request name is required"
        ))
    );
    assert!(entries.borrow().is_empty());
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

#[test]
fn GivenConfigurationRepository_WhenConfigurationIsLoaded_ThenService_ShouldValidateAndNormalizeKey(
) {
    let service = ConfigurationService::new(TestConfigurationRepository);

    let result = service.load_configuration_value(" theme ");

    assert_eq!(result, Ok(Some("dark".to_owned())));
}

#[test]
fn GivenBlankConfigurationKey_WhenConfigurationIsLoaded_ThenService_ShouldReturnValidationError() {
    let service = ConfigurationService::new(TestConfigurationRepository);

    let result = service.load_configuration_value(" ");

    assert_eq!(
        result,
        Err(ServiceError::InvalidConfiguration(
            "configuration key is required"
        ))
    );
}
