use std::cell::RefCell;
use std::rc::Rc;

use sql_intelliscan_services::{
    contracts::{ConnectionRepository, ConnectionRepositoryFactory},
    errors::{DataAccessError, DataAccessResult},
};

#[derive(Clone, Debug)]
pub struct MockConnectionRepository {
    result: DataAccessResult<bool>,
}

impl MockConnectionRepository {
    pub fn succeeds() -> Self {
        Self { result: Ok(true) }
    }

    pub fn rejects_connection() -> Self {
        Self { result: Ok(false) }
    }

    pub fn fails_with(error: DataAccessError) -> Self {
        Self { result: Err(error) }
    }
}

impl ConnectionRepository for MockConnectionRepository {
    async fn validate_connection(&self) -> DataAccessResult<bool> {
        self.result.clone()
    }
}

#[derive(Clone, Debug)]
pub struct MockConnectionRepositoryFactory {
    build_result: DataAccessResult<MockConnectionRepository>,
    requested_connection_strings: Rc<RefCell<Vec<String>>>,
}

impl MockConnectionRepositoryFactory {
    pub fn builds(repository: MockConnectionRepository) -> Self {
        Self {
            build_result: Ok(repository),
            requested_connection_strings: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn fails_with(error: DataAccessError) -> Self {
        Self {
            build_result: Err(error),
            requested_connection_strings: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn requested_connection_strings(&self) -> Vec<String> {
        self.requested_connection_strings.borrow().clone()
    }
}

impl ConnectionRepositoryFactory for MockConnectionRepositoryFactory {
    type Repository = MockConnectionRepository;

    fn build(&self, connection_string: &str) -> DataAccessResult<Self::Repository> {
        self.requested_connection_strings
            .borrow_mut()
            .push(connection_string.to_owned());
        self.build_result.clone()
    }
}
