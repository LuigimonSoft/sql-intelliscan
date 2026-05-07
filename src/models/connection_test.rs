use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackendConnectionTestResult {
    pub success: bool,
    pub message: String,
    pub server_version: Option<String>,
    pub database: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConnectionTestRequest {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub encrypt: bool,
    pub trust_server_certificate: bool,
    pub connection_timeout_seconds: u64,
    pub application_name: Option<String>,
}

impl Default for ConnectionTestRequest {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 1433,
            database: "master".to_string(),
            username: String::new(),
            password: String::new(),
            encrypt: true,
            trust_server_certificate: false,
            connection_timeout_seconds: 30,
            application_name: Some("SQL Intelliscan".to_string()),
        }
    }
}

impl fmt::Debug for ConnectionTestRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectionTestRequest")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("database", &self.database)
            .field("username", &self.username)
            .field("password", &"***")
            .field("encrypt", &self.encrypt)
            .field("trust_server_certificate", &self.trust_server_certificate)
            .field(
                "connection_timeout_seconds",
                &self.connection_timeout_seconds,
            )
            .field("application_name", &self.application_name)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub message: String,
    pub server_version: Option<String>,
    pub database: Option<String>,
    pub latency_ms: Option<u64>,
}

impl From<BackendConnectionTestResult> for ConnectionTestResult {
    fn from(response: BackendConnectionTestResult) -> Self {
        Self {
            success: response.success,
            message: response.message,
            server_version: response.server_version,
            database: response.database,
            latency_ms: response.latency_ms,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConnectionTestError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionTestStatus {
    Idle,
    Loading,
    Success(ConnectionTestResult),
    Error(ConnectionTestError),
}
