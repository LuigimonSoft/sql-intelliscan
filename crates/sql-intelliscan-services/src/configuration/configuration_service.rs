use crate::{contracts::ConfigurationRepository, errors::ServiceError};

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

    pub fn load_configuration_value(&self, key: &str) -> Result<Option<String>, ServiceError> {
        let normalized_key = key.trim();
        if normalized_key.is_empty() {
            return Err(ServiceError::InvalidConfiguration(
                "configuration key is required",
            ));
        }

        Ok(self.repository.find_value(normalized_key))
    }

    pub fn get_configuration_value(&self, key: &str) -> Option<String> {
        self.repository.find_value(key)
    }
}
