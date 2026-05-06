use crate::{AppState, CommandErrorResponse, ConnectionTestResponse};

pub async fn test_connection(
    state: tauri::State<'_, AppState>,
) -> Result<ConnectionTestResponse, CommandErrorResponse> {
    test_connection_with_state(state.inner()).await
}

pub async fn test_connection_with_state(
    state: &AppState,
) -> Result<ConnectionTestResponse, CommandErrorResponse> {
    state
        .test_connection()
        .await
        .map(ConnectionTestResponse::from)
        .map_err(CommandErrorResponse::from_service_error)
}
