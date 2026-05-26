#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    SourceUnavailable,
    InvalidConfiguration(&'static str),
    QueryExecutionFailed(String),
    ResultMappingFailed(&'static str),
}

impl RepositoryError {
    pub fn safe_category(&self) -> &'static str {
        match self {
            Self::SourceUnavailable => "CONNECTION_FAILED",
            Self::InvalidConfiguration(_) => "INVALID_CONFIGURATION",
            Self::QueryExecutionFailed(message) if is_timeout_failure(message) => "TIMEOUT",
            Self::QueryExecutionFailed(_) => "QUERY_VALIDATION_FAILED",
            Self::ResultMappingFailed(_) => "QUERY_VALIDATION_FAILED",
        }
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

fn is_timeout_failure(message: &str) -> bool {
    let normalized = message.to_ascii_lowercase();

    normalized.contains("timeout") || normalized.contains("timed out")
}
