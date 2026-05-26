#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    SourceUnavailable,
    InvalidConfiguration(&'static str),
    QueryExecutionFailed(String),
    ResultMappingFailed(&'static str),
}

impl RepositoryError {
    pub(crate) fn safe_category(&self) -> &'static str {
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::RepositoryError;

    #[test]
    fn GivenRepositoryErrors_WhenSafeCategoryIsRequested_ThenResult_ShouldReturnDiagnosticCategories(
    ) {
        assert_eq!(
            RepositoryError::InvalidConfiguration("missing password").safe_category(),
            "INVALID_CONFIGURATION"
        );
        assert_eq!(
            RepositoryError::SourceUnavailable.safe_category(),
            "CONNECTION_FAILED"
        );
        assert_eq!(
            RepositoryError::QueryExecutionFailed("connection timeout".to_owned()).safe_category(),
            "TIMEOUT"
        );
        assert_eq!(
            RepositoryError::QueryExecutionFailed("validation failed".to_owned()).safe_category(),
            "QUERY_VALIDATION_FAILED"
        );
        assert_eq!(
            RepositoryError::ResultMappingFailed("unexpected scalar type").safe_category(),
            "QUERY_VALIDATION_FAILED"
        );
    }
}
