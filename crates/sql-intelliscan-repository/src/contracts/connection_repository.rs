#![allow(async_fn_in_trait)]

use crate::errors::RepositoryResult;

pub trait ConnectionRepository {
    async fn validate_connection(&self) -> RepositoryResult<bool>;
}
