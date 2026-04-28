use crate::{contracts::ConnectionRepository, errors::ServiceResult, models::ConnectionTestResult};

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

    pub async fn test_connection(&self) -> ServiceResult<ConnectionTestResult> {
        let is_valid = self.repository.validate_connection().await?;

        Ok(if is_valid {
            ConnectionTestResult::valid()
        } else {
            ConnectionTestResult::invalid()
        })
    }

    pub async fn validate_connection(&self) -> ServiceResult<bool> {
        self.test_connection().await.map(|result| result.is_valid)
    }
}
