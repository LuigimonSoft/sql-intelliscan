use mssqlrust::{dataset::DataValue, execute_scalar, infrastructure::mssql::MssqlConfig, Command};

use crate::{
    contracts::ConnectionRepository,
    errors::{RepositoryError, RepositoryResult},
    models::SqlServerConnectionConfig,
};

#[allow(async_fn_in_trait)]
pub trait MssqlScalarClient: Send + Sync {
    async fn execute_scalar(
        &self,
        config: MssqlConfig,
        command: Command,
    ) -> Result<Option<DataValue>, String>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultMssqlScalarClient;

impl MssqlScalarClient for DefaultMssqlScalarClient {
    async fn execute_scalar(
        &self,
        config: MssqlConfig,
        command: Command,
    ) -> Result<Option<DataValue>, String> {
        execute_scalar(config, command)
            .await
            .map_err(|err| err.to_string())
    }
}

pub struct SqlServerConnectionRepository<C = DefaultMssqlScalarClient>
where
    C: MssqlScalarClient,
{
    config: SqlServerConnectionConfig,
    client: C,
}

impl SqlServerConnectionRepository<DefaultMssqlScalarClient> {
    pub fn new(config: SqlServerConnectionConfig) -> Self {
        Self {
            config,
            client: DefaultMssqlScalarClient,
        }
    }
}

impl<C> SqlServerConnectionRepository<C>
where
    C: MssqlScalarClient,
{
    pub fn with_client(config: SqlServerConnectionConfig, client: C) -> Self {
        Self { config, client }
    }

    fn to_mssql_config(&self) -> MssqlConfig {
        MssqlConfig::new(
            &self.config.host,
            self.config.port,
            &self.config.username,
            &self.config.password,
            &self.config.database,
            self.config.trust_cert,
        )
    }

    fn map_validation_result(result: Option<DataValue>) -> RepositoryResult<bool> {
        match result {
            Some(DataValue::Int(value)) => Ok(value == 1),
            Some(DataValue::TinyInt(value)) => Ok(value == 1),
            Some(DataValue::SmallInt(value)) => Ok(value == 1),
            Some(DataValue::BigInt(value)) => Ok(value == 1),
            Some(DataValue::Null) | None => {
                Err(RepositoryError::ResultMappingFailed("empty scalar result"))
            }
            _ => Err(RepositoryError::ResultMappingFailed(
                "unexpected scalar type",
            )),
        }
    }
}

impl<C> ConnectionRepository for SqlServerConnectionRepository<C>
where
    C: MssqlScalarClient,
{
    async fn validate_connection(&self) -> RepositoryResult<bool> {
        let command = Command::query("SELECT 1");

        let scalar = self
            .client
            .execute_scalar(self.to_mssql_config(), command)
            .await
            .map_err(RepositoryError::QueryExecutionFailed)?;

        Self::map_validation_result(scalar)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use mssqlrust::{dataset::DataValue, infrastructure::mssql::MssqlConfig, Command};

    use crate::{
        contracts::ConnectionRepository,
        errors::RepositoryError,
        models::SqlServerConnectionConfig,
        sql_server::connection_repository::{MssqlScalarClient, SqlServerConnectionRepository},
    };

    struct SuccessClient;

    impl MssqlScalarClient for SuccessClient {
        async fn execute_scalar(
            &self,
            _config: MssqlConfig,
            _command: Command,
        ) -> Result<Option<DataValue>, String> {
            Ok(Some(DataValue::Int(1)))
        }
    }

    struct InvalidTypeClient;

    impl MssqlScalarClient for InvalidTypeClient {
        async fn execute_scalar(
            &self,
            _config: MssqlConfig,
            _command: Command,
        ) -> Result<Option<DataValue>, String> {
            Ok(Some(DataValue::Text("online".to_owned())))
        }
    }

    struct FailingClient;

    impl MssqlScalarClient for FailingClient {
        async fn execute_scalar(
            &self,
            _config: MssqlConfig,
            _command: Command,
        ) -> Result<Option<DataValue>, String> {
            Err("tcp timeout".to_owned())
        }
    }

    fn build_config() -> SqlServerConnectionConfig {
        SqlServerConnectionConfig::from_connection_string(
            "Server=localhost,1433;User Id=sa;Password=secret;Database=master;TrustServerCertificate=true;",
        )
        .expect("valid config")
    }

    #[test]
    fn GivenSqlServerConnectionString_WhenParsed_ThenConfig_ShouldBeCreated() {
        let config = SqlServerConnectionConfig::from_connection_string(
            "Server=localhost,1433;User Id=sa;Password=secret;Database=master;TrustServerCertificate=true;",
        )
        .expect("config should parse");

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 1433);
        assert_eq!(config.username, "sa");
        assert_eq!(config.password, "secret");
        assert_eq!(config.database, "master");
        assert!(config.trust_cert);
    }

    #[test]
    fn GivenMockClient_WhenValidationIsRequested_ThenRepository_ShouldReturnTrue() {
        let repository = SqlServerConnectionRepository::with_client(build_config(), SuccessClient);

        let result = futures::executor::block_on(repository.validate_connection());

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn GivenUnexpectedScalarType_WhenValidationIsRequested_ThenRepository_ShouldMapError() {
        let repository =
            SqlServerConnectionRepository::with_client(build_config(), InvalidTypeClient);

        let result = futures::executor::block_on(repository.validate_connection());

        assert_eq!(
            result,
            Err(RepositoryError::ResultMappingFailed(
                "unexpected scalar type"
            ))
        );
    }

    #[test]
    fn GivenMssqlClientFailure_WhenValidationIsRequested_ThenRepository_ShouldMapError() {
        let repository = SqlServerConnectionRepository::with_client(build_config(), FailingClient);

        let result = futures::executor::block_on(repository.validate_connection());

        assert_eq!(
            result,
            Err(RepositoryError::QueryExecutionFailed(
                "tcp timeout".to_owned()
            ))
        );
    }
}
