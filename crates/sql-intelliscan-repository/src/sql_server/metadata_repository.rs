use crate::contracts::BackendMetadataRepository;

#[derive(Debug, Default, Clone, Copy)]
pub struct SqlServerMetadataRepository;

impl BackendMetadataRepository for SqlServerMetadataRepository {
    fn origin(&self) -> &'static str {
        "SQL Server"
    }
}
