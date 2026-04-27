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

    pub fn validate_connection(&self, connection_id: &str) -> bool {
        self.repository.can_connect(connection_id)
    }
}
