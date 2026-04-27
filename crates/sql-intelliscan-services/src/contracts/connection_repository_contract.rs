pub trait ConnectionRepository {
    fn can_connect(&self, connection_id: &str) -> bool;
}
