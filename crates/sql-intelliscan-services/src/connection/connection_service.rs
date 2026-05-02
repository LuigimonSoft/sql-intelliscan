use crate::{
    contracts::{ConnectionRepository, ConnectionRepositoryFactory},
    errors::ServiceResult,
    models::ConnectionTestResult,
};

#[derive(Debug, Clone)]
pub struct ConnectionService<R> {
    repository: R,
}

impl<R> ConnectionService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ConnectionService<R>
where
    R: ConnectionRepository,
{
    pub async fn test_connection(&self) -> ServiceResult<ConnectionTestResult> {
        self.repository
            .validate_connection()
            .await
            .map_err(Into::into)
    }

    pub async fn validate_connection(&self) -> ServiceResult<bool> {
        self.test_connection().await.map(|result| result.is_valid)
    }
}

impl<F> ConnectionService<F>
where
    F: ConnectionRepositoryFactory,
{
    pub async fn test_configured_connection(
        &self,
        connection_string: &str,
    ) -> ServiceResult<ConnectionTestResult> {
        let repository = self.repository.build(connection_string)?;
        repository.validate_connection().await.map_err(Into::into)
    }

    pub async fn validate_configured_connection(
        &self,
        connection_string: &str,
    ) -> ServiceResult<bool> {
        self.test_configured_connection(connection_string)
            .await
            .map(|result| result.is_valid)
    }
}
