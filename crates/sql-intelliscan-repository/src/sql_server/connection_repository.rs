use mssqlrust::{dataset::DataValue, execute_scalar, infrastructure::mssql::MssqlConfig, Command};

use crate::{
    contracts::ConnectionRepository,
    errors::{RepositoryError, RepositoryResult},
    models::SqlServerConnectionConfig,
};

enum MssqlScalarClient {
    Default,
    #[cfg(test)]
    Test(Box<dyn TestMssqlScalarClient>),
}

impl MssqlScalarClient {
    async fn execute_scalar(
        &self,
        config: MssqlConfig,
        command: Command,
    ) -> Result<Option<DataValue>, String> {
        match self {
            Self::Default => execute_scalar(config, command)
                .await
                .map_err(|err| err.to_string()),
            #[cfg(test)]
            Self::Test(client) => client.execute_scalar(config, command),
        }
    }
}

#[cfg(test)]
trait TestMssqlScalarClient: Send + Sync {
    fn execute_scalar(
        &self,
        config: MssqlConfig,
        command: Command,
    ) -> Result<Option<DataValue>, String>;
}

pub struct SqlServerConnectionRepository {
    config: SqlServerConnectionConfig,
    client: MssqlScalarClient,
}

impl SqlServerConnectionRepository {
    pub fn new(config: SqlServerConnectionConfig) -> Self {
        Self {
            config,
            client: MssqlScalarClient::Default,
        }
    }

    #[cfg(test)]
    fn with_client<C>(config: SqlServerConnectionConfig, client: C) -> Self
    where
        C: TestMssqlScalarClient + 'static,
    {
        Self {
            config,
            client: MssqlScalarClient::Test(Box::new(client)),
        }
    }

    fn to_mssql_config(&self) -> MssqlConfig {
        // mssqlrust 1.0.2 exposes only these connection options. The richer
        // SqlServerConnectionConfig keeps unsupported settings parsed for
        // forward compatibility without leaking them outside the repository.
        MssqlConfig::new(
            &self.config.host,
            self.config.port,
            &self.config.username,
            &self.config.password,
            &self.config.database,
            self.config.trust_server_certificate,
        )
    }

    async fn execute_validation_query(&self) -> RepositoryResult<Option<DataValue>> {
        let command = Command::query("SELECT 1");

        self.client
            .execute_scalar(self.to_mssql_config(), command)
            .await
            .map_err(Self::map_driver_error)
    }

    fn map_driver_error(error: String) -> RepositoryError {
        let normalized = error.to_ascii_lowercase();

        if normalized.contains("login failed")
            || normalized.contains("authentication")
            || normalized.contains("password")
        {
            return RepositoryError::InvalidConfiguration("authentication failed");
        }

        if normalized.contains("timeout") || normalized.contains("timed out") {
            return RepositoryError::QueryExecutionFailed("connection timeout".to_owned());
        }

        if normalized.contains("network")
            || normalized.contains("tcp")
            || normalized.contains("connection refused")
            || normalized.contains("could not connect")
            || normalized.contains("unreachable")
        {
            return RepositoryError::SourceUnavailable;
        }

        RepositoryError::QueryExecutionFailed("SQL Server validation query failed".to_owned())
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

impl ConnectionRepository for SqlServerConnectionRepository {
    async fn validate_connection(&self) -> RepositoryResult<bool> {
        let scalar = self.execute_validation_query().await?;

        Self::map_validation_result(scalar)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use mssqlrust::{dataset::DataValue, infrastructure::mssql::MssqlConfig, Command};

    use crate::{
        contracts::ConnectionRepository, errors::RepositoryError, models::SqlServerConnectionConfig,
    };

    use super::{SqlServerConnectionRepository, TestMssqlScalarClient};

    struct SuccessClient;

    impl TestMssqlScalarClient for SuccessClient {
        fn execute_scalar(
            &self,
            _config: MssqlConfig,
            _command: Command,
        ) -> Result<Option<DataValue>, String> {
            Ok(Some(DataValue::Int(1)))
        }
    }

    struct InvalidTypeClient;

    impl TestMssqlScalarClient for InvalidTypeClient {
        fn execute_scalar(
            &self,
            _config: MssqlConfig,
            _command: Command,
        ) -> Result<Option<DataValue>, String> {
            Ok(Some(DataValue::Text("online".to_owned())))
        }
    }

    struct FailingClient;

    impl TestMssqlScalarClient for FailingClient {
        fn execute_scalar(
            &self,
            _config: MssqlConfig,
            _command: Command,
        ) -> Result<Option<DataValue>, String> {
            Err("tcp timeout".to_owned())
        }
    }

    struct GenericFailingClient;

    impl TestMssqlScalarClient for GenericFailingClient {
        fn execute_scalar(
            &self,
            _config: MssqlConfig,
            _command: Command,
        ) -> Result<Option<DataValue>, String> {
            Err("server returned secret connection string details".to_owned())
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
        assert!(config.trust_server_certificate);
    }

    #[test]
    fn GivenSqlServerConfig_WhenRepositoryIsCreated_ThenDriverConfig_ShouldBeBuilt() {
        let repository = SqlServerConnectionRepository::new(build_config());

        let _driver_config = repository.to_mssql_config();
    }

    #[test]
    fn GivenTinyIntScalar_WhenValidationResultIsMapped_ThenResult_ShouldReturnTrue() {
        let result =
            SqlServerConnectionRepository::map_validation_result(Some(DataValue::TinyInt(1)));

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn GivenSmallIntScalar_WhenValidationResultIsMapped_ThenResult_ShouldReturnTrue() {
        let result =
            SqlServerConnectionRepository::map_validation_result(Some(DataValue::SmallInt(1)));

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn GivenBigIntScalar_WhenValidationResultIsMapped_ThenResult_ShouldReturnTrue() {
        let result =
            SqlServerConnectionRepository::map_validation_result(Some(DataValue::BigInt(1)));

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn GivenEmptyScalar_WhenValidationResultIsMapped_ThenResult_ShouldReturnMappingError() {
        let result = SqlServerConnectionRepository::map_validation_result(None);

        assert_eq!(
            result,
            Err(RepositoryError::ResultMappingFailed("empty scalar result"))
        );
    }

    #[test]
    fn GivenNullScalar_WhenValidationResultIsMapped_ThenResult_ShouldReturnMappingError() {
        let result = SqlServerConnectionRepository::map_validation_result(Some(DataValue::Null));

        assert_eq!(
            result,
            Err(RepositoryError::ResultMappingFailed("empty scalar result"))
        );
    }

    #[test]
    fn GivenMockClient_WhenValidationIsRequested_ThenRepository_ShouldReturnTrue() {
        let repository = SqlServerConnectionRepository::with_client(build_config(), SuccessClient);

        let result =
            futures::executor::block_on(ConnectionRepository::validate_connection(&repository));

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn GivenUnexpectedScalarType_WhenValidationIsRequested_ThenRepository_ShouldMapError() {
        let repository =
            SqlServerConnectionRepository::with_client(build_config(), InvalidTypeClient);

        let result =
            futures::executor::block_on(ConnectionRepository::validate_connection(&repository));

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

        let result =
            futures::executor::block_on(ConnectionRepository::validate_connection(&repository));

        assert_eq!(
            result,
            Err(RepositoryError::QueryExecutionFailed(
                "connection timeout".to_owned()
            ))
        );
    }

    #[test]
    fn GivenDriverFailureWithSensitiveDetails_WhenValidationIsRequested_ThenRepository_ShouldNotExposeDetails(
    ) {
        let repository =
            SqlServerConnectionRepository::with_client(build_config(), GenericFailingClient);

        let result =
            futures::executor::block_on(ConnectionRepository::validate_connection(&repository));

        assert_eq!(
            result,
            Err(RepositoryError::QueryExecutionFailed(
                "SQL Server validation query failed".to_owned()
            ))
        );
    }

    #[test]
    #[ignore = "Requires the real SQL Server CI fixture from scripts/sql-server/real-test-setup.sql"]
    fn GivenRealSqlServerTable_WhenScalarIsRequested_ThenClient_ShouldReturnSeededCount() {
        let connection_string = std::env::var("SQLSERVER_TEST_CONNECTION_STRING")
            .expect("SQLSERVER_TEST_CONNECTION_STRING must be provided");
        let config = SqlServerConnectionConfig::from_connection_string(&connection_string)
            .expect("valid SQL Server connection string");
        let repository = SqlServerConnectionRepository::new(config);
        let runtime = tokio::runtime::Runtime::new().expect("Tokio runtime should start");

        let scalar = runtime
            .block_on(repository.client.execute_scalar(
                repository.to_mssql_config(),
                Command::query("SELECT COUNT_BIG(*) FROM dbo.IntelliscanScalarSmokeItems"),
            ))
            .expect("scalar query should succeed");

        assert_eq!(scalar, Some(DataValue::BigInt(3)));
    }
}
