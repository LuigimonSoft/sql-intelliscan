use sql_intelliscan_common::backend_origin;

use crate::contracts::BackendMetadataRepository;

#[derive(Debug, Default, Clone, Copy)]
pub struct StaticBackendMetadataRepository;

impl BackendMetadataRepository for StaticBackendMetadataRepository {
    fn origin(&self) -> &'static str {
        backend_origin()
    }
}
