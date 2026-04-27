pub const BACKEND_ORIGIN: &str = "Rust";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuditEvent<'a> {
    pub actor: &'a str,
    pub action: &'a str,
    pub resource: &'a str,
}

pub trait ConnectionFactory {
    type Connection;
    type Error;

    fn create_connection(&self) -> Result<Self::Connection, Self::Error>;
}

pub trait SqlServerConnector {
    type Connection;
    type Error;

    fn connect(&self, connection_string: &str) -> Result<Self::Connection, Self::Error>;
}

pub trait Logger {
    fn log(&self, level: LogLevel, message: &str);
}

pub trait AuditApplicationService {
    type Error;

    fn record_event(&self, event: AuditEvent<'_>) -> Result<(), Self::Error>;
}

pub fn backend_origin() -> &'static str {
    BACKEND_ORIGIN
}
