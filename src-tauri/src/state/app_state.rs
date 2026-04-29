use std::sync::Arc;

use sql_intelliscan_services::GreetingService;

use crate::dependency_wiring::BackendMetadataRepositoryAdapter;

type GreetingAppService = GreetingService<BackendMetadataRepositoryAdapter>;

#[derive(Clone)]
pub struct AppState {
    greeting_service: Arc<GreetingAppService>,
}

impl AppState {
    pub(crate) fn new(greeting_service: Arc<GreetingAppService>) -> Self {
        Self { greeting_service }
    }

    pub fn greet(&self, name: &str) -> String {
        self.greeting_service.greet(name)
    }
}
