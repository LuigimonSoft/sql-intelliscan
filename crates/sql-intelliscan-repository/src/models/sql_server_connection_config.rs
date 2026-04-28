use crate::errors::{RepositoryError, RepositoryResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlServerConnectionConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub trust_cert: bool,
}

impl SqlServerConnectionConfig {
    pub fn from_connection_string(connection_string: &str) -> RepositoryResult<Self> {
        let mut host = None;
        let mut port = 1433u16;
        let mut username = None;
        let mut password = None;
        let mut database = None;
        let mut trust_cert = false;

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
                    host = host_parts
                        .next()
                        .map(str::trim)
                        .filter(|item| !item.is_empty())
                        .map(str::to_owned);
                    if let Some(parsed_port) = host_parts
                        .next()
                        .and_then(|item| item.trim().parse::<u16>().ok())
                    {
                        port = parsed_port;
                    }
                }
                "user id" | "uid" | "user" => username = Some(value.to_owned()),
                "password" | "pwd" => password = Some(value.to_owned()),
                "database" | "initial catalog" => database = Some(value.to_owned()),
                "trustservercertificate" => {
                    trust_cert = matches!(value.to_ascii_lowercase().as_str(), "true" | "1" | "yes")
                }
                _ => {}
            }
        }

        Ok(Self {
            host: host.ok_or(RepositoryError::InvalidConfiguration("missing host"))?,
            port,
            username: username.ok_or(RepositoryError::InvalidConfiguration("missing username"))?,
            password: password.ok_or(RepositoryError::InvalidConfiguration("missing password"))?,
            database: database.ok_or(RepositoryError::InvalidConfiguration("missing database"))?,
            trust_cert,
        })
    }
}
