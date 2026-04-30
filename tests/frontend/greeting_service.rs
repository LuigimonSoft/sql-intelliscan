#![allow(non_snake_case)]

use sql_intelliscan_ui::services::greeting_service::map_greet_response;
use sql_intelliscan_ui::services::tauri_client::{CommandErrorResponse, CommandSuccessResponse};

#[test]
fn GivenSuccessfulGreetResponse_WhenMapped_ThenMessage_ShouldUseBackendData() {
    let message = map_greet_response(Ok(CommandSuccessResponse {
        message: "Greeting generated successfully".to_string(),
        data: "Hello, Frontend! You've been greeted from Rust!".to_string(),
    }));

    assert_eq!(message, "Hello, Frontend! You've been greeted from Rust!");
}

#[test]
fn GivenFailedGreetResponse_WhenMapped_ThenMessage_ShouldExposeBackendError() {
    let message = map_greet_response(Err(CommandErrorResponse {
        message: "The backend returned an unexpected response.".to_string(),
    }));

    assert_eq!(message, "The backend returned an unexpected response.");
}
