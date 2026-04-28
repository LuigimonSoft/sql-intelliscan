pub trait BackendMetadataRepository {
    fn origin(&self) -> &'static str;
}
