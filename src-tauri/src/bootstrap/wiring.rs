use std::sync::Arc;

use sql_intelliscan_services::{
    repository_wiring::{BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory},
    ConnectionService, GreetingService,
};

use crate::state::{AppState, AppStateResult};

pub fn build_app_state() -> AppStateResult {
    let backend_metadata_repository = BackendMetadataRepositoryAdapter::default_static();
    let greeting_service = Arc::new(GreetingService::new(backend_metadata_repository));

    let command_connection_repository_factory = SqlServerConnectionRepositoryFactory;
    let command_connection_service = Arc::new(ConnectionService::new(
        command_connection_repository_factory,
    ));

    Ok(AppState::new(greeting_service, command_connection_service))
}
