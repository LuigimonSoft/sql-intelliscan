use std::sync::OnceLock;

use sql_intelliscan_services::errors::ServiceError;

use crate::bootstrap::wiring;
use crate::state::AppStateResult;

static SHARED_APP_STATE: OnceLock<AppStateResult> = OnceLock::new();

pub fn build_app_state() -> AppStateResult {
    wiring::build_app_state()
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
