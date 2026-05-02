#![allow(non_snake_case)]

mod test_utils;

use std::sync::Arc;

use sql_intelliscan_services::{
    contracts::ConnectionRepositoryContract,
    errors::{DataAccessError, ServiceError},
    models::ConnectionTestResult,
    AuditService, ConfigurationService, ConnectionService, GreetingService,
};

use test_utils::{
    MockAuditRepository, MockBackendMetadataRepository, MockConfigurationRepository,
    MockConnectionRepository, MockConnectionRepositoryFactory,
};

#[test]
fn GivenRepositoryDouble_WhenGreetingIsRequested_ThenService_ShouldUseRepositoryOrigin() {
    let repository = MockBackendMetadataRepository::with_origin("TestBackend");
    let service = GreetingService::new(repository);

    assert_eq!(
        service.greet("Carlos"),
        "Hello, Carlos! You've been greeted from TestBackend!"
    );
}

#[test]
fn GivenConnectionRepository_WhenValidationIsRequested_ThenService_ShouldDelegateConnectivityCheck()
{
    let service = ConnectionService::new(MockConnectionRepository::succeeds());

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Ok(ConnectionTestResult::valid()));
}

#[test]
fn GivenConnectionRepository_WhenBooleanValidationIsRequested_ThenService_ShouldReturnBoolean() {
    let service = ConnectionService::new(MockConnectionRepository::succeeds());

    let result = futures::executor::block_on(service.validate_connection());

    assert_eq!(result, Ok(true));
}

#[test]
fn GivenConnectionRepositoryContract_WhenStoredBehindArc_ThenContract_ShouldBeSendSyncCompatible() {
    let repository: Arc<dyn ConnectionRepositoryContract + Send + Sync> =
        Arc::new(MockConnectionRepository::succeeds());

    let result = futures::executor::block_on(repository.test_connection());

    assert_eq!(result, Ok(ConnectionTestResult::valid()));
}

#[test]
fn GivenConnectionRepositoryContractMock_WhenConnectionFails_ThenContract_ShouldReturnServiceError()
{
    let repository: Arc<dyn ConnectionRepositoryContract + Send + Sync> = Arc::new(
        MockConnectionRepository::fails_with(DataAccessError::SourceUnavailable),
    );

    let result = futures::executor::block_on(repository.test_connection());

    assert_eq!(result, Err(ServiceError::SourceUnavailable));
}

#[test]
fn GivenConnectionRepositoryReturnsFalse_WhenValidationIsRequested_ThenService_ShouldReturnInvalidResult(
) {
    let service = ConnectionService::new(MockConnectionRepository::rejects_connection());

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Ok(ConnectionTestResult::invalid()));
}

#[test]
fn GivenConnectionDetails_WhenResultIsCreated_ThenModel_ShouldExposeSafeMetadata() {
    let result = ConnectionTestResult::valid_with_details(Some("master".to_owned()), Some(42));

    assert!(result.is_valid);
    assert_eq!(result.message, "Connection successful");
    assert_eq!(result.database, Some("master".to_owned()));
    assert_eq!(result.latency_ms, Some(42));
}

#[test]
fn GivenInvalidConnectionResult_WhenCreated_ThenModel_ShouldUseFailureMessage() {
    let result = ConnectionTestResult::invalid();

    assert!(!result.is_valid);
    assert_eq!(result.message, "Connection failed");
    assert_eq!(result.database, None);
    assert_eq!(result.latency_ms, None);
}

#[test]
fn GivenConnectionRepositoryFailure_WhenValidationIsRequested_ThenService_ShouldReturnServiceError()
{
    let service = ConnectionService::new(MockConnectionRepository::fails_with(
        DataAccessError::SourceUnavailable,
    ));

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Err(ServiceError::SourceUnavailable));
}

#[test]
fn GivenRepositoryQueryFailure_WhenValidationIsRequested_ThenService_ShouldNormalizeError() {
    let service = ConnectionService::new(MockConnectionRepository::fails_with(
        DataAccessError::QueryExecutionFailed("timeout".to_owned()),
    ));

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(result, Err(ServiceError::QueryExecutionFailed));
}

#[test]
fn GivenRepositoryMappingFailure_WhenValidationIsRequested_ThenService_ShouldNormalizeError() {
    let service = ConnectionService::new(MockConnectionRepository::fails_with(
        DataAccessError::ResultMappingFailed("unexpected scalar type"),
    ));

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(
        result,
        Err(ServiceError::ResultMappingFailed("unexpected scalar type"))
    );
}

#[test]
fn GivenRepositoryConfigurationFailure_WhenValidationIsRequested_ThenService_ShouldNormalizeError()
{
    let service = ConnectionService::new(MockConnectionRepository::fails_with(
        DataAccessError::InvalidConfiguration("missing host"),
    ));

    let result = futures::executor::block_on(service.test_connection());

    assert_eq!(
        result,
        Err(ServiceError::InvalidConfiguration("missing host"))
    );
}

#[test]
fn GivenConnectionRepositoryFactory_WhenConfiguredValidationIsRequested_ThenService_ShouldBuildRepository(
) {
    let repository = MockConnectionRepository::succeeds();
    let factory = MockConnectionRepositoryFactory::builds(repository);
    let service = ConnectionService::new(factory.clone());

    let result =
        futures::executor::block_on(service.test_configured_connection("Server=localhost"));

    assert_eq!(result, Ok(ConnectionTestResult::valid()));
    assert_eq!(
        factory.requested_connection_strings(),
        ["Server=localhost".to_owned()]
    );
}

#[test]
fn GivenInvalidConnectionConfiguration_WhenConfiguredValidationIsRequested_ThenService_ShouldReturnValidationError(
) {
    let factory = MockConnectionRepositoryFactory::fails_with(
        DataAccessError::InvalidConfiguration("connection string is required"),
    );
    let service = ConnectionService::new(factory.clone());

    let result = futures::executor::block_on(service.test_configured_connection(" "));

    assert_eq!(
        result,
        Err(ServiceError::InvalidConfiguration(
            "connection string is required"
        ))
    );
    assert_eq!(factory.requested_connection_strings(), [" ".to_owned()]);
}

#[test]
fn GivenConnectionRepositoryFactory_WhenConfiguredBooleanValidationIsRequested_ThenService_ShouldReturnBoolean(
) {
    let repository = MockConnectionRepository::rejects_connection();
    let factory = MockConnectionRepositoryFactory::builds(repository);
    let service = ConnectionService::new(factory);

    let result =
        futures::executor::block_on(service.validate_configured_connection("Server=localhost"));

    assert_eq!(result, Ok(false));
}

#[test]
fn GivenAuditRepository_WhenAuditEntryIsRegistered_ThenService_ShouldPersistEntryThroughRepository()
{
    let repository = MockAuditRepository::default();
    let service = AuditService::new(repository.clone());

    let result = service.start_audit_execution(" User logged in ");

    assert_eq!(result, Ok(()));
    assert_eq!(repository.entries(), ["User logged in"]);
}

#[test]
fn GivenAuditRepository_WhenRawAuditEntryIsRegistered_ThenService_ShouldPersistEntryThroughRepository(
) {
    let repository = MockAuditRepository::default();
    let service = AuditService::new(repository.clone());

    service.register_audit_entry("User logged in");

    assert_eq!(repository.entries(), ["User logged in"]);
}

#[test]
fn GivenBlankAuditRequest_WhenAuditExecutionStarts_ThenService_ShouldReturnValidationError() {
    let repository = MockAuditRepository::default();
    let service = AuditService::new(repository.clone());

    let result = service.start_audit_execution(" ");

    assert_eq!(
        result,
        Err(ServiceError::InvalidAuditRequest(
            "request name is required"
        ))
    );
    assert!(repository.entries().is_empty());
}

#[test]
fn GivenConfigurationRepository_WhenValueIsRequested_ThenService_ShouldReturnValueFromRepository() {
    let repository = MockConfigurationRepository::default().with_value("theme", "dark");
    let service = ConfigurationService::new(repository);

    assert_eq!(
        service.get_configuration_value("theme"),
        Some("dark".to_owned())
    );
    assert_eq!(service.get_configuration_value("timezone"), None);
}

#[test]
fn GivenConfigurationRepository_WhenConfigurationIsLoaded_ThenService_ShouldValidateAndNormalizeKey(
) {
    let repository = MockConfigurationRepository::default().with_value("theme", "dark");
    let service = ConfigurationService::new(repository);

    let result = service.load_configuration_value(" theme ");

    assert_eq!(result, Ok(Some("dark".to_owned())));
}

#[test]
fn GivenBlankConfigurationKey_WhenConfigurationIsLoaded_ThenService_ShouldReturnValidationError() {
    let service = ConfigurationService::new(MockConfigurationRepository::default());

    let result = service.load_configuration_value(" ");

    assert_eq!(
        result,
        Err(ServiceError::InvalidConfiguration(
            "configuration key is required"
        ))
    );
}
