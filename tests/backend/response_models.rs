#![allow(non_snake_case)]

use sql_intelliscan_lib::{CommandErrorResponse, CommandSuccessResponse, ServiceError};

#[test]
fn GivenServiceErrors_WhenMappedToCommandErrorResponse_ThenMessages_ShouldBeUserFriendly() {
    let cases = [
        (
            ServiceError::InvalidAuditRequest("missing target"),
            "The submitted audit request is invalid.",
        ),
        (
            ServiceError::InvalidConfiguration("missing password"),
            "The provided configuration is invalid: missing password.",
        ),
        (
            ServiceError::InvalidName,
            "The provided name is invalid.",
        ),
        (
            ServiceError::QueryExecutionFailed,
            "The operation failed while querying the data source.",
        ),
        (
            ServiceError::ResultMappingFailed("unexpected scalar"),
            "The operation could not map the returned data.",
        ),
        (
            ServiceError::SourceUnavailable,
            "The data source is currently unavailable.",
        ),
    ];

    for (error, expected_message) in cases {
        let response = CommandErrorResponse::from_service_error(error);

        assert_eq!(response.message, expected_message);
    }
}

#[test]
fn GivenCommandSuccessResponse_WhenBuilt_ThenFields_ShouldContainMessageAndData() {
    let response = CommandSuccessResponse {
        message: "Connection validated successfully".to_string(),
        data: true,
    };

    assert_eq!(response.message, "Connection validated successfully");
    assert!(response.data);
}
