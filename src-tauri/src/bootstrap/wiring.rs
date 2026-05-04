use std::{future::Future, pin::Pin, sync::Arc, time::Instant};

use sql_intelliscan_repository::{
    ConnectionRepository as RepositoryConnectionRepository, RepositoryError, RepositoryResult,
    SqlServerConnectionConfig, SqlServerConnectionRepository,
};
use sql_intelliscan_services::{
    contracts::ConnectionRepository,
    errors::{DataAccessError, DataAccessResult, ServiceError},
    models::ConnectionTestResult,
    repository_wiring::{BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory},
    ConnectionService, GreetingService,
};

use crate::{
    config::connection_config_loader::load_connection_config,
    state::{AppState, AppStateResult},
};

type RepositoryValidationFuture<'a> =
    Pin<Box<dyn Future<Output = RepositoryResult<bool>> + Send + 'a>>;

trait StartupSqlServerConnectionRepository: Send + Sync {
    fn validate_connection(&self) -> RepositoryValidationFuture<'_>;
}

impl StartupSqlServerConnectionRepository for SqlServerConnectionRepository {
    fn validate_connection(&self) -> RepositoryValidationFuture<'_> {
        Box::pin(RepositoryConnectionRepository::validate_connection(self))
    }
}

struct StartupConnectionRepositoryAdapter {
    repository: Box<dyn StartupSqlServerConnectionRepository>,
    database: String,
}

impl StartupConnectionRepositoryAdapter {
    fn new(config: SqlServerConnectionConfig) -> Self {
        let database = config.database.clone();
        let repository = SqlServerConnectionRepository::new(config);

        Self {
            repository: Box::new(repository),
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

impl ConnectionRepository for StartupConnectionRepositoryAdapter {
    #[allow(clippy::manual_async_fn)]
    fn validate_connection(
        &self,
    ) -> impl Future<Output = DataAccessResult<ConnectionTestResult>> + Send {
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

pub fn build_app_state() -> AppStateResult {
    let connection_config = load_connection_config()?;

    build_app_state_with_connection_config(connection_config)
}

pub fn build_app_state_with_connection_config(
    connection_config: SqlServerConnectionConfig,
) -> AppStateResult {
    connection_config
        .validate()
        .map_err(|errors| ServiceError::InvalidConfiguration(validation_error_message(&errors)))?;

    let backend_metadata_repository = BackendMetadataRepositoryAdapter::default_static();
    let greeting_service = Arc::new(GreetingService::new(backend_metadata_repository));

    let startup_connection_repository = StartupConnectionRepositoryAdapter::new(connection_config);
    let startup_connection_service =
        Arc::new(ConnectionService::new(startup_connection_repository));

    let command_connection_repository_factory = SqlServerConnectionRepositoryFactory;
    let command_connection_service = Arc::new(ConnectionService::new(
        command_connection_repository_factory,
    ));

    Ok(AppState::with_startup_connection_service(
        greeting_service,
        command_connection_service,
        startup_connection_service,
    ))
}

fn validation_error_message(
    errors: &[sql_intelliscan_repository::ConnectionConfigValidationError],
) -> &'static str {
    match errors.first() {
        Some(sql_intelliscan_repository::ConnectionConfigValidationError::HostRequired) => {
            "missing host"
        }
        Some(sql_intelliscan_repository::ConnectionConfigValidationError::DatabaseRequired) => {
            "missing database"
        }
        Some(sql_intelliscan_repository::ConnectionConfigValidationError::UsernameRequired) => {
            "missing username"
        }
        Some(sql_intelliscan_repository::ConnectionConfigValidationError::PasswordRequired) => {
            "missing password"
        }
        Some(sql_intelliscan_repository::ConnectionConfigValidationError::InvalidPort) => {
            "invalid port"
        }
        Some(sql_intelliscan_repository::ConnectionConfigValidationError::InvalidTimeout) => {
            "invalid timeout"
        }
        Some(
            sql_intelliscan_repository::ConnectionConfigValidationError::InvalidApplicationName,
        ) => "invalid application name",
        None => "invalid SQL Server connection configuration",
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
