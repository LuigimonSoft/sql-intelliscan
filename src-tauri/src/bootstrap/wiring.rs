use std::sync::Arc;

use sql_intelliscan_repository::SqlServerConnectionConfig;
use sql_intelliscan_services::{
    errors::ServiceError,
    repository_wiring::{
        BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryAdapter,
        SqlServerConnectionRepositoryFactory,
    },
    ConnectionService, GreetingService,
};

use crate::{
    config::connection_config_loader::load_connection_config,
    state::{AppState, AppStateResult},
};

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

    let startup_connection_repository =
        SqlServerConnectionRepositoryAdapter::from_config(connection_config);
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
