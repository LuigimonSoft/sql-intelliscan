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
            DataAccessError::QueryExecutionFailed(_) => Self::QueryExecutionFailed,
            DataAccessError::ResultMappingFailed(reason) => Self::ResultMappingFailed(reason),
        }
    }
}
