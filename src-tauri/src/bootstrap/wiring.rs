use std::sync::Arc;

use sql_intelliscan_services::{
    repository_wiring::{BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory},
    ConnectionService, GreetingService,
};

use crate::{
    config::connection_config_loader::CONNECTION_STRING_ENV_VAR,
    state::{AppState, AppStateResult, ConfiguredConnectionService},
};

pub fn build_app_state() -> AppStateResult {
    let backend_metadata_repository = BackendMetadataRepositoryAdapter::default_static();
    let greeting_service = Arc::new(GreetingService::new(backend_metadata_repository));

    let command_connection_service = Arc::new(ConfiguredConnectionService::new(
        ConnectionService::new(SqlServerConnectionRepositoryFactory),
        std::env::var(CONNECTION_STRING_ENV_VAR).ok(),
    ));

    Ok(AppState::new(greeting_service, command_connection_service))
}
