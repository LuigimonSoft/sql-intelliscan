use sql_intelliscan_common::backend_origin;

pub trait BackendMetadataRepository {
    fn origin(&self) -> &'static str;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StaticBackendMetadataRepository;

impl BackendMetadataRepository for StaticBackendMetadataRepository {
    fn origin(&self) -> &'static str {
        backend_origin()
    }
}
