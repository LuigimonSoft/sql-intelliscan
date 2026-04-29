#![allow(async_fn_in_trait)]

use crate::errors::DataAccessResult;

pub trait ConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool>;
}

pub trait ConnectionRepositoryFactory {
    type Repository: ConnectionRepository;

    fn build(&self, connection_string: &str) -> DataAccessResult<Self::Repository>;
}
