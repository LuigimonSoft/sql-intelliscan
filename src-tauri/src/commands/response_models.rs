use serde::Serialize;

use sql_intelliscan_services::errors::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommandSuccessResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommandErrorResponse {
    pub message: String,
}

impl CommandErrorResponse {
    pub fn from_service_error(error: ServiceError) -> Self {
        let message = match error {
            ServiceError::InvalidAuditRequest(_) => {
                "The submitted audit request is invalid.".to_string()
            }
            ServiceError::InvalidConfiguration(reason) => {
                format!("The provided configuration is invalid: {reason}.")
            }
            ServiceError::InvalidName => "The provided name is invalid.".to_string(),
            ServiceError::QueryExecutionFailed => {
                "The operation failed while querying the data source.".to_string()
            }
            ServiceError::ResultMappingFailed(_) => {
                "The operation could not map the returned data.".to_string()
            }
            ServiceError::SourceUnavailable => {
                "The data source is currently unavailable.".to_string()
            }
        };

        Self { message }
    }
}
