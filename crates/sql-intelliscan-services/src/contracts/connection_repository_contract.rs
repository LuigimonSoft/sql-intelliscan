use std::future::Future;

use async_trait::async_trait;

use crate::{
    errors::{DataAccessResult, ServiceResult},
    models::ConnectionTestResult,
};

#[async_trait]
pub trait ConnectionRepositoryContract: Send + Sync {
    async fn test_connection(&self) -> ServiceResult<ConnectionTestResult>;
}

#[async_trait]
impl<T> ConnectionRepositoryContract for T
where
    T: ConnectionRepository + Send + Sync,
{
    async fn test_connection(&self) -> ServiceResult<ConnectionTestResult> {
        self.validate_connection().await.map_err(Into::into)
    }
}

pub trait ConnectionRepository {
    fn validate_connection(
        &self,
    ) -> impl Future<Output = DataAccessResult<ConnectionTestResult>> + Send;
}

pub trait ConnectionRepositoryFactory {
    type Repository: ConnectionRepository;

    fn build(&self, connection_string: &str) -> DataAccessResult<Self::Repository>;
}
