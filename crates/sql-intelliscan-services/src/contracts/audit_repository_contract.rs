pub trait AuditRepository {
    fn save_entry(&self, entry: &str);
}
