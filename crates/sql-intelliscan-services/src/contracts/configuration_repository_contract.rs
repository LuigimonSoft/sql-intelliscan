pub trait ConfigurationRepository {
    fn find_value(&self, key: &str) -> Option<String>;
}
