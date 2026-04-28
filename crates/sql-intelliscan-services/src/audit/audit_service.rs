use crate::{contracts::AuditRepository, errors::ServiceError};

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

    pub fn start_audit_execution(&self, request_name: &str) -> Result<(), ServiceError> {
        let normalized_request_name = request_name.trim();
        if normalized_request_name.is_empty() {
            return Err(ServiceError::InvalidAuditRequest(
                "request name is required",
            ));
        }

        self.repository.save_entry(normalized_request_name);

        Ok(())
    }

    pub fn register_audit_entry(&self, entry: &str) {
        self.repository.save_entry(entry);
    }
}
