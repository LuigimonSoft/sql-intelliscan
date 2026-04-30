use sql_intelliscan_services::contracts::BackendMetadataRepository;

#[derive(Clone, Copy, Debug)]
pub struct MockBackendMetadataRepository {
    origin: &'static str,
}

impl MockBackendMetadataRepository {
    pub fn with_origin(origin: &'static str) -> Self {
        Self { origin }
    }
}

impl BackendMetadataRepository for MockBackendMetadataRepository {
    fn origin(&self) -> &'static str {
        self.origin
    }
}
