#![allow(non_snake_case)]

use sql_intelliscan_lib::{CommandErrorResponse, CommandSuccessResponse, ServiceError};

#[test]
fn GivenServiceErrors_WhenMappedToCommandErrorResponse_ThenCodesAndMessages_ShouldBeFrontendSafe() {
    let cases = [
        (
            ServiceError::InvalidAuditRequest("missing target"),
            "INVALID_CONFIGURATION",
            "The SQL Server connection configuration is invalid.",
        ),
        (
            ServiceError::InvalidConfiguration("missing password"),
            "INVALID_CONFIGURATION",
            "The SQL Server connection configuration is invalid.",
        ),
        (ServiceError::InvalidName, "INVALID_CONFIGURATION", "The provided name is invalid."),
        (
            ServiceError::QueryExecutionFailed,
            "CONNECTION_FAILED",
            "Unable to connect to the SQL Server instance.",
        ),
        (
            ServiceError::ResultMappingFailed("unexpected scalar"),
            "UNEXPECTED_ERROR",
            "An unexpected error occurred while testing the connection.",
        ),
        (
            ServiceError::SourceUnavailable,
            "CONNECTION_FAILED",
            "Unable to connect to the SQL Server instance.",
        ),
    ];

    for (error, expected_code, expected_message) in cases {
        let response = CommandErrorResponse::from_service_error(error);

        assert_eq!(response.code, expected_code);
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
