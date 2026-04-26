pub mod data_access;
pub mod errors;
pub mod models;

pub use data_access::StaticBackendMetadataRepository;

pub mod contracts {
    pub trait BackendMetadataRepository {
        fn origin(&self) -> &'static str;
    }
}

pub use contracts::BackendMetadataRepository;
