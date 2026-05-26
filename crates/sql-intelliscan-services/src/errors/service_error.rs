use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataAccessError {
    SourceUnavailable,
    InvalidConfiguration(&'static str),
    QueryExecutionFailed(String),
    ResultMappingFailed(&'static str),
}

pub type DataAccessResult<T> = Result<T, DataAccessError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ServiceError {
    InvalidAuditRequest(&'static str),
    InvalidConfiguration(&'static str),
    InvalidName,
    ConnectionTimeout,
    QueryExecutionFailed,
    ResultMappingFailed(&'static str),
    SourceUnavailable,
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl From<DataAccessError> for ServiceError {
    fn from(error: DataAccessError) -> Self {
        match error {
            DataAccessError::SourceUnavailable => Self::SourceUnavailable,
            DataAccessError::InvalidConfiguration(reason) => Self::InvalidConfiguration(reason),
            DataAccessError::QueryExecutionFailed(reason) if is_timeout_reason(&reason) => {
                Self::ConnectionTimeout
            }
            DataAccessError::QueryExecutionFailed(_) => Self::QueryExecutionFailed,
            DataAccessError::ResultMappingFailed(reason) => Self::ResultMappingFailed(reason),
        }
    }
}

impl ServiceError {
    pub fn safe_category(&self) -> &'static str {
        match self {
            Self::InvalidAuditRequest(_) => "INVALID_AUDIT_REQUEST",
            Self::InvalidConfiguration(_) => "INVALID_CONFIGURATION",
            Self::InvalidName => "INVALID_NAME",
            Self::ConnectionTimeout => "CONNECTION_TIMEOUT",
            Self::QueryExecutionFailed => "QUERY_EXECUTION_FAILED",
            Self::ResultMappingFailed(_) => "RESULT_MAPPING_FAILED",
            Self::SourceUnavailable => "SOURCE_UNAVAILABLE",
        }
    }
}

fn is_timeout_reason(reason: &str) -> bool {
    let normalized = reason.to_ascii_lowercase();

    normalized.contains("timeout") || normalized.contains("timed out")
}
