use crate::contracts::ConfigurationRepository;

#[derive(Debug, Clone)]
pub struct ConfigurationService<R> {
    repository: R,
}

impl<R> ConfigurationService<R>
where
    R: ConfigurationRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn get_configuration_value(&self, key: &str) -> Option<String> {
        self.repository.find_value(key)
    }
}
