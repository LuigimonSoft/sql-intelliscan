use std::collections::HashMap;

use sql_intelliscan_services::contracts::ConfigurationRepository;

#[derive(Clone, Debug, Default)]
pub struct MockConfigurationRepository {
    values: HashMap<String, String>,
}

impl MockConfigurationRepository {
    pub fn with_value(mut self, key: &str, value: &str) -> Self {
        self.values.insert(key.to_owned(), value.to_owned());
        self
    }
}

impl ConfigurationRepository for MockConfigurationRepository {
    fn find_value(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }
}
