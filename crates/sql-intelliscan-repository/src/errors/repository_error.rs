#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    SourceUnavailable,
    InvalidConfiguration(&'static str),
    QueryExecutionFailed(String),
    ResultMappingFailed(&'static str),
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
