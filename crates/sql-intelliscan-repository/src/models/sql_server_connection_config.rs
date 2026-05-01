use std::fmt;

use crate::errors::{RepositoryError, RepositoryResult};

const DEFAULT_PORT: u16 = 1433;
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
const MAX_TIMEOUT_SECONDS: u64 = 300;
const DEFAULT_APPLICATION_NAME: &str = "SQL Intelliscan";

#[derive(Clone, PartialEq, Eq)]
pub struct SqlServerConnectionConfig {
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

impl fmt::Debug for SqlServerConnectionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SqlServerConnectionConfig")
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

impl Default for SqlServerConnectionConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_owned(),
            port: DEFAULT_PORT,
            database: "master".to_owned(),
            username: String::new(),
            password: String::new(),
            encrypt: true,
            trust_server_certificate: false,
            connection_timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
            application_name: Some(DEFAULT_APPLICATION_NAME.to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionConfigValidationError {
    HostRequired,
    DatabaseRequired,
    UsernameRequired,
    PasswordRequired,
    InvalidPort,
    InvalidTimeout,
    InvalidApplicationName,
}

impl ConnectionConfigValidationError {
    fn as_repository_message(&self) -> &'static str {
        match self {
            Self::HostRequired => "missing host",
            Self::DatabaseRequired => "missing database",
            Self::UsernameRequired => "missing username",
            Self::PasswordRequired => "missing password",
            Self::InvalidPort => "invalid port",
            Self::InvalidTimeout => "invalid timeout",
            Self::InvalidApplicationName => "invalid application name",
        }
    }
}

impl SqlServerConnectionConfig {
    pub fn validate(&self) -> Result<(), Vec<ConnectionConfigValidationError>> {
        let mut errors = Vec::new();

        if self.host.trim().is_empty() {
            errors.push(ConnectionConfigValidationError::HostRequired);
        }

        if self.database.trim().is_empty() {
            errors.push(ConnectionConfigValidationError::DatabaseRequired);
        }

        if self.username.trim().is_empty() {
            errors.push(ConnectionConfigValidationError::UsernameRequired);
        }

        if self.password.trim().is_empty() {
            errors.push(ConnectionConfigValidationError::PasswordRequired);
        }

        if self.port == 0 {
            errors.push(ConnectionConfigValidationError::InvalidPort);
        }

        if self.connection_timeout_seconds == 0
            || self.connection_timeout_seconds > MAX_TIMEOUT_SECONDS
        {
            errors.push(ConnectionConfigValidationError::InvalidTimeout);
        }

        if let Some(application_name) = &self.application_name {
            if application_name.trim().is_empty() {
                errors.push(ConnectionConfigValidationError::InvalidApplicationName);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn from_connection_string(connection_string: &str) -> RepositoryResult<Self> {
        let mut config = Self {
            host: String::new(),
            database: String::new(),
            ..Self::default()
        };

        for token in connection_string
            .split(';')
            .filter(|segment| !segment.trim().is_empty())
        {
            let mut parts = token.splitn(2, '=');
            let key = parts.next().unwrap_or_default().trim().to_lowercase();
            let value = parts
                .next()
                .ok_or(RepositoryError::InvalidConfiguration(
                    "invalid connection string segment",
                ))?
                .trim();

            match key.as_str() {
                "server" | "data source" => {
                    let mut host_parts = value.split(',');
                    config.host = host_parts.next().unwrap_or_default().trim().to_owned();
                    if let Some(parsed_port) = host_parts
                        .next()
                        .and_then(|item| item.trim().parse::<u16>().ok())
                    {
                        config.port = parsed_port;
                    }
                }
                "user id" | "uid" | "user" => config.username = value.to_owned(),
                "password" | "pwd" => config.password = value.to_owned(),
                "database" | "initial catalog" => config.database = value.to_owned(),
                "trustservercertificate" => {
                    config.trust_server_certificate =
                        matches!(value.to_ascii_lowercase().as_str(), "true" | "1" | "yes")
                }
                "encrypt" => {
                    config.encrypt =
                        !matches!(value.to_ascii_lowercase().as_str(), "false" | "0" | "no")
                }
                "connection timeout" | "connect timeout" => {
                    if let Ok(parsed_timeout) = value.parse::<u64>() {
                        config.connection_timeout_seconds = parsed_timeout;
                    }
                }
                "application name" => {
                    config.application_name = if value.is_empty() {
                        None
                    } else {
                        Some(value.to_owned())
                    }
                }
                _ => {}
            }
        }

        config.validate().map_err(|errors| {
            let first = errors
                .first()
                .map(ConnectionConfigValidationError::as_repository_message)
                .unwrap_or("invalid SQL Server connection configuration");
            RepositoryError::InvalidConfiguration(first)
        })?;

        Ok(config)
    }
}
