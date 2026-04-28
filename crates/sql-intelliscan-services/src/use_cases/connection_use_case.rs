use sql_intelliscan_repository::RepositoryResult;

use crate::contracts::ConnectionRepository;

#[derive(Debug, Clone)]
pub struct ConnectionService<R> {
    repository: R,
}

impl<R> ConnectionService<R>
where
    R: ConnectionRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn validate_connection(&self) -> RepositoryResult<bool> {
        self.repository.validate_connection().await
    }
}
