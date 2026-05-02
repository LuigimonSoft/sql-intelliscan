use std::sync::{Arc, OnceLock};

use sql_intelliscan_services::{
    errors::ServiceError,
    repository_wiring::{BackendMetadataRepositoryAdapter, SqlServerConnectionRepositoryFactory},
    ConnectionService, GreetingService,
};

use crate::state::{AppState, AppStateResult};

static SHARED_APP_STATE: OnceLock<AppStateResult> = OnceLock::new();

pub fn build_app_state() -> AppStateResult {
    let backend_metadata_repository = BackendMetadataRepositoryAdapter::default_static();
    let sql_server_connection_repository_factory = SqlServerConnectionRepositoryFactory;

    let greeting_service = Arc::new(GreetingService::new(backend_metadata_repository));
    let connection_service = Arc::new(ConnectionService::new(
        sql_server_connection_repository_factory,
    ));

    Ok(AppState::new(greeting_service, connection_service))
}

pub fn shared_app_state() -> AppStateResult {
    SHARED_APP_STATE.get_or_init(build_app_state).clone()
}

pub fn greet_user(name: &str) -> Result<String, ServiceError> {
    Ok(shared_app_state()?.greet(name))
}

pub async fn validate_sql_server_connection(
    connection_string: &str,
) -> Result<sql_intelliscan_services::models::ConnectionTestResult, ServiceError> {
    shared_app_state()?
        .validate_sql_server_connection(connection_string)
        .await
}
