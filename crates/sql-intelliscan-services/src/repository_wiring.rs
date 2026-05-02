use std::{future::Future, pin::Pin, time::Instant};

use sql_intelliscan_repository::{
    BackendMetadataRepository as RepositoryBackendMetadataRepository,
    ConnectionRepository as RepositoryConnectionRepository, RepositoryError, RepositoryResult,
    SqlServerConnectionConfig, SqlServerConnectionRepository, StaticBackendMetadataRepository,
};

use crate::{
    contracts::{
        BackendMetadataRepository as ServiceBackendMetadataRepository, ConnectionRepository,
        ConnectionRepositoryFactory,
    },
    errors::{DataAccessError, DataAccessResult},
    models::ConnectionTestResult,
};

type RepositoryValidationFuture<'a> =
    Pin<Box<dyn Future<Output = RepositoryResult<bool>> + Send + 'a>>;

trait SqlServerConnectionValidator: Send + Sync {
    fn validate_connection(&self) -> RepositoryValidationFuture<'_>;
}

impl SqlServerConnectionValidator for SqlServerConnectionRepository {
    fn validate_connection(&self) -> RepositoryValidationFuture<'_> {
        Box::pin(RepositoryConnectionRepository::validate_connection(self))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BackendMetadataRepositoryAdapter(pub StaticBackendMetadataRepository);

impl BackendMetadataRepositoryAdapter {
    pub fn default_static() -> Self {
        Self(StaticBackendMetadataRepository)
    }
}

impl ServiceBackendMetadataRepository for BackendMetadataRepositoryAdapter {
    fn origin(&self) -> &'static str {
        self.0.origin()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SqlServerConnectionRepositoryFactory;

impl ConnectionRepositoryFactory for SqlServerConnectionRepositoryFactory {
    type Repository = SqlServerConnectionRepositoryAdapter;

    fn build(&self, connection_string: &str) -> DataAccessResult<Self::Repository> {
        let config = SqlServerConnectionConfig::from_connection_string(connection_string)
            .map_err(map_repository_error_to_data_access)?;

        Ok(SqlServerConnectionRepositoryAdapter::new(config))
    }
}

pub struct SqlServerConnectionRepositoryAdapter {
    repository: Box<dyn SqlServerConnectionValidator>,
    database: String,
}

impl SqlServerConnectionRepositoryAdapter {
    fn new(config: SqlServerConnectionConfig) -> Self {
        let database = config.database.clone();

        Self::with_validator(database, SqlServerConnectionRepository::new(config))
    }

    fn with_validator<V>(database: String, validator: V) -> Self
    where
        V: SqlServerConnectionValidator + 'static,
    {
        Self {
            repository: Box::new(validator),
            database,
        }
    }

    fn elapsed_millis(started_at: Instant) -> u64 {
        started_at
            .elapsed()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX)
    }
}

impl ConnectionRepository for SqlServerConnectionRepositoryAdapter {
    #[allow(clippy::manual_async_fn)]
    fn validate_connection(
        &self,
    ) -> impl std::future::Future<Output = DataAccessResult<ConnectionTestResult>> + Send {
        async move {
            let started_at = Instant::now();
            let is_valid = self
                .repository
                .validate_connection()
                .await
                .map_err(map_repository_error_to_data_access)?;

            Ok(if is_valid {
                ConnectionTestResult::valid_with_details(
                    Some(self.database.clone()),
                    Some(Self::elapsed_millis(started_at)),
                )
            } else {
                ConnectionTestResult::invalid()
            })
        }
    }
}

fn map_repository_error_to_data_access(error: RepositoryError) -> DataAccessError {
    match error {
        RepositoryError::SourceUnavailable => DataAccessError::SourceUnavailable,
        RepositoryError::InvalidConfiguration(reason) => {
            DataAccessError::InvalidConfiguration(reason)
        }
        RepositoryError::QueryExecutionFailed(reason) => {
            DataAccessError::QueryExecutionFailed(reason)
        }
        RepositoryError::ResultMappingFailed(reason) => {
            DataAccessError::ResultMappingFailed(reason)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    struct ValidatorDouble {
        result: RepositoryResult<bool>,
    }

    impl ValidatorDouble {
        fn succeeds() -> Self {
            Self { result: Ok(true) }
        }

        fn rejects() -> Self {
            Self { result: Ok(false) }
        }

        fn fails_with(error: RepositoryError) -> Self {
            Self { result: Err(error) }
        }
    }

    impl SqlServerConnectionValidator for ValidatorDouble {
        fn validate_connection(&self) -> RepositoryValidationFuture<'_> {
            let result = self.result.clone();

            Box::pin(async move { result })
        }
    }

    #[test]
    fn GivenSuccessfulRepository_WhenValidationRuns_ThenResult_ShouldIncludeSafeDetails() {
        let adapter = SqlServerConnectionRepositoryAdapter::with_validator(
            "master".to_owned(),
            ValidatorDouble::succeeds(),
        );

        let result =
            futures::executor::block_on(ConnectionRepository::validate_connection(&adapter))
                .expect("validation should succeed");

        assert!(result.is_valid);
        assert_eq!(result.database, Some("master".to_owned()));
        assert!(result.latency_ms.is_some());
    }

    #[test]
    fn GivenRejectedRepository_WhenValidationRuns_ThenResult_ShouldBeInvalid() {
        let adapter = SqlServerConnectionRepositoryAdapter::with_validator(
            "master".to_owned(),
            ValidatorDouble::rejects(),
        );

        let result =
            futures::executor::block_on(ConnectionRepository::validate_connection(&adapter))
                .expect("validation should return an invalid result");

        assert_eq!(result, ConnectionTestResult::invalid());
    }

    #[test]
    fn GivenRepositoryFailure_WhenValidationRuns_ThenError_ShouldBeMapped() {
        let adapter = SqlServerConnectionRepositoryAdapter::with_validator(
            "master".to_owned(),
            ValidatorDouble::fails_with(RepositoryError::ResultMappingFailed(
                "unexpected scalar type",
            )),
        );

        let result =
            futures::executor::block_on(ConnectionRepository::validate_connection(&adapter));

        assert_eq!(
            result,
            Err(DataAccessError::ResultMappingFailed(
                "unexpected scalar type"
            ))
        );
    }

    #[test]
    fn GivenRepositoryErrors_WhenMapped_ThenServiceErrors_ShouldRemainSafe() {
        assert_eq!(
            map_repository_error_to_data_access(RepositoryError::SourceUnavailable),
            DataAccessError::SourceUnavailable
        );
        assert_eq!(
            map_repository_error_to_data_access(RepositoryError::InvalidConfiguration(
                "missing host"
            )),
            DataAccessError::InvalidConfiguration("missing host")
        );
        assert_eq!(
            map_repository_error_to_data_access(RepositoryError::QueryExecutionFailed(
                "connection timeout".to_owned()
            )),
            DataAccessError::QueryExecutionFailed("connection timeout".to_owned())
        );
    }
}
