use crate::contracts::AuditRepository;

#[derive(Debug, Clone)]
pub struct AuditService<R> {
    repository: R,
}

impl<R> AuditService<R>
where
    R: AuditRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn register_audit_entry(&self, entry: &str) {
        self.repository.save_entry(entry);
    }
}
