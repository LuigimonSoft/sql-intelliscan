use sql_intelliscan_repository::BackendMetadataRepository;
use sql_intelliscan_repository::StaticBackendMetadataRepository;

#[derive(Debug, Clone, Copy)]
pub struct GreetingService<R> {
    repository: R,
}

impl<R> GreetingService<R>
where
    R: BackendMetadataRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn greet(&self, name: &str) -> String {
        format!(
            "Hello, {}! You've been greeted from {}!",
            name,
            self.repository.origin()
        )
    }
}

pub fn greet(name: &str) -> String {
    GreetingService::new(StaticBackendMetadataRepository).greet(name)
}
