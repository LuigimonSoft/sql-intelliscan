use std::{fmt, time::Instant};

use serde::Deserialize;
use tracing::{debug, info, warn};

use crate::{AppState, CommandErrorResponse, ConnectionTestResponse, ServiceError};

const COMMAND_LOG_TARGET: &str = "sql_intelliscan::commands";
const TEST_CONNECTION_COMMAND: &str = "test_connection";

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct ConnectionTestRequest {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub encrypt: bool,
    pub trust_server_certificate: bool,
    pub connection_timeout_seconds: u64,
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
            .finish()
    }
}

pub async fn test_connection(
    state: tauri::State<'_, AppState>,
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResponse, CommandErrorResponse> {
    test_connection_with_state(state.inner(), request).await
}

pub async fn test_connection_with_state(
    state: &AppState,
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResponse, CommandErrorResponse> {
    let started_at = Instant::now();

    info!(
        target: COMMAND_LOG_TARGET,
        command = TEST_CONNECTION_COMMAND,
        "Tauri command started"
    );

    debug!(
        target: COMMAND_LOG_TARGET,
        command = TEST_CONNECTION_COMMAND,
        port = request.port,
        timeout_seconds = request.connection_timeout_seconds,
        encrypt = request.encrypt,
        trust_server_certificate = request.trust_server_certificate,
        "Received connection test request metadata"
    );

    let connection_string = match connection_string_from_request(&request) {
        Ok(connection_string) => connection_string,
        Err(error) => {
            let command_error = CommandErrorResponse::from_service_error(error);
            warn!(
                target: COMMAND_LOG_TARGET,
                command = TEST_CONNECTION_COMMAND,
                error_code = %command_error.code,
                elapsed_ms = started_at.elapsed().as_millis(),
                "Tauri command failed"
            );
            return Err(command_error);
        }
    };

    match state
        .test_connection_with_connection_string(&connection_string)
        .await
        .map(ConnectionTestResponse::from)
        .map_err(CommandErrorResponse::from_service_error)
    {
        Ok(response) => {
            info!(
                target: COMMAND_LOG_TARGET,
                command = TEST_CONNECTION_COMMAND,
                elapsed_ms = started_at.elapsed().as_millis(),
                "Tauri command completed successfully"
            );
            Ok(response)
        }
        Err(command_error) => {
            warn!(
                target: COMMAND_LOG_TARGET,
                command = TEST_CONNECTION_COMMAND,
                error_code = %command_error.code,
                elapsed_ms = started_at.elapsed().as_millis(),
                "Tauri command failed"
            );
            Err(command_error)
        }
    }
}

pub fn connection_string_from_request(
    request: &ConnectionTestRequest,
) -> Result<String, ServiceError> {
    if [
        request.host.as_str(),
        request.database.as_str(),
        request.username.as_str(),
        request.password.as_str(),
    ]
    .iter()
    .any(|value| value.contains(';'))
    {
        return Err(ServiceError::InvalidConfiguration(
            "connection fields must not contain semicolons",
        ));
    }

    Ok(format!(
        "Server={},{};Database={};User Id={};Password={};Encrypt={};TrustServerCertificate={};Connection Timeout={}",
        request.host.trim(),
        request.port,
        request.database.trim(),
        request.username.trim(),
        request.password,
        request.encrypt,
        request.trust_server_certificate,
        request.connection_timeout_seconds
    ))
}
