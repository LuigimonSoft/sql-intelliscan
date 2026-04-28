#![allow(async_fn_in_trait)]

use crate::errors::DataAccessResult;

pub trait ConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool>;
}
