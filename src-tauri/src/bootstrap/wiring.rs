use std::sync::Arc;

use sql_intelliscan_services::{
    errors::ServiceResult,
    repository_wiring::{BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory},
    ConnectionService, GreetingService,
};

use crate::{
    config::connection_config_loader::{
        load_connection_config_from_env_value, CONNECTION_STRING_ENV_VAR,
    },
    state::{AppState, AppStateResult, ConfiguredConnectionService},
};

pub fn build_app_state() -> AppStateResult {
    let backend_metadata_repository = BackendMetadataRepositoryAdapter::default_static();
    let greeting_service = Arc::new(GreetingService::new(backend_metadata_repository));

    let command_connection_service = Arc::new(ConfiguredConnectionService::new(
        ConnectionService::new(SqlServerConnectionRepositoryFactory),
        configured_connection_string_from_env_value(std::env::var(CONNECTION_STRING_ENV_VAR))?,
    ));

    Ok(AppState::new(greeting_service, command_connection_service))
}

pub fn configured_connection_string_from_env_value(
    configured_connection_string: Result<String, std::env::VarError>,
) -> ServiceResult<Option<String>> {
    match configured_connection_string {
        Ok(connection_string) => {
            load_connection_config_from_env_value(Ok(connection_string.clone()))?;

            Ok(Some(connection_string))
        }
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(error @ std::env::VarError::NotUnicode(_)) => {
            load_connection_config_from_env_value(Err(error))?;

            unreachable!("non-unicode environment values always return an error")
        }
    }
}
