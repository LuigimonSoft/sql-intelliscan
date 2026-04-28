use std::sync::Arc;

use sql_intelliscan_services::GreetingService;

use crate::dependency_wiring::BackendMetadataRepositoryAdapter;

pub type GreetingAppService = GreetingService<BackendMetadataRepositoryAdapter>;

#[derive(Clone)]
pub struct AppState {
    pub greeting_service: Arc<GreetingAppService>,
}

impl AppState {
    pub fn new(greeting_service: Arc<GreetingAppService>) -> Self {
        Self { greeting_service }
    }
}
