#![allow(non_snake_case)]

use std::cell::RefCell;

use sql_intelliscan_common::{
    backend_origin, AuditApplicationService, AuditEvent, ConnectionFactory, LogLevel, Logger,
    SqlServerConnector, BACKEND_ORIGIN,
};

#[derive(Debug, PartialEq, Eq)]
struct TestConnection(&'static str);

#[derive(Default)]
struct TestConnectionFactory;

impl ConnectionFactory for TestConnectionFactory {
    type Connection = TestConnection;
    type Error = &'static str;

    fn create_connection(&self) -> Result<Self::Connection, Self::Error> {
        Ok(TestConnection("factory-connection"))
    }
}

#[derive(Default)]
struct TestSqlServerConnector {
    requested_connection_string: RefCell<Option<String>>,
}

impl SqlServerConnector for TestSqlServerConnector {
    type Connection = TestConnection;
    type Error = &'static str;

    fn connect(&self, connection_string: &str) -> Result<Self::Connection, Self::Error> {
        *self.requested_connection_string.borrow_mut() = Some(connection_string.to_owned());

        Ok(TestConnection("sql-server-connection"))
    }
}

#[derive(Default)]
struct TestLogger {
    entries: RefCell<Vec<(LogLevel, String)>>,
}

impl Logger for TestLogger {
    fn log(&self, level: LogLevel, message: &str) {
        self.entries.borrow_mut().push((level, message.to_owned()));
    }
}

#[derive(Default)]
struct TestAuditApplicationService {
    events: RefCell<Vec<String>>,
}

impl AuditApplicationService for TestAuditApplicationService {
    type Error = &'static str;

    fn record_event(&self, event: AuditEvent<'_>) -> Result<(), Self::Error> {
        self.events.borrow_mut().push(format!(
            "{}:{}:{}",
            event.actor, event.action, event.resource
        ));

        Ok(())
    }
}

#[test]
fn GivenCommonLayer_WhenBackendOriginIsRequested_ThenValue_ShouldExposeSharedBackendOrigin() {
    assert_eq!(backend_origin(), BACKEND_ORIGIN);
    assert_eq!(BACKEND_ORIGIN, "Rust");
}

#[test]
fn GivenConnectionFactoryDouble_WhenConnectionIsCreated_ThenTrait_ShouldReturnFactoryConnection() {
    let connection = TestConnectionFactory
        .create_connection()
        .expect("connection should be created");

    assert_eq!(connection, TestConnection("factory-connection"));
}

#[test]
fn GivenSqlServerConnectorDouble_WhenConnectionIsRequested_ThenTrait_ShouldReceiveConnectionString()
{
    let connector = TestSqlServerConnector::default();

    let connection = connector
        .connect("server=localhost;database=master")
        .expect("connection should be created");

    assert_eq!(connection, TestConnection("sql-server-connection"));
    assert_eq!(
        connector.requested_connection_string.borrow().as_deref(),
        Some("server=localhost;database=master")
    );
}

#[test]
fn GivenLoggerDouble_WhenMessageIsLogged_ThenTrait_ShouldCaptureLevelAndMessage() {
    let logger = TestLogger::default();

    logger.log(LogLevel::Info, "application started");

    assert_eq!(
        logger.entries.borrow().as_slice(),
        &[(LogLevel::Info, "application started".to_owned())]
    );
}

#[test]
fn GivenAuditApplicationServiceDouble_WhenEventIsRecorded_ThenTrait_ShouldCaptureAuditFields() {
    let service = TestAuditApplicationService::default();

    service
        .record_event(AuditEvent {
            actor: "carlos",
            action: "scan",
            resource: "sql-server",
        })
        .expect("audit event should be recorded");

    assert_eq!(
        service.events.borrow().as_slice(),
        &["carlos:scan:sql-server".to_owned()]
    );
}
