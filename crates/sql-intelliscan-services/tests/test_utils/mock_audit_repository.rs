use std::cell::RefCell;
use std::rc::Rc;

use sql_intelliscan_services::contracts::AuditRepository;

#[derive(Clone, Debug, Default)]
pub struct MockAuditRepository {
    entries: Rc<RefCell<Vec<String>>>,
}

impl MockAuditRepository {
    pub fn entries(&self) -> Vec<String> {
        self.entries.borrow().clone()
    }
}

impl AuditRepository for MockAuditRepository {
    fn save_entry(&self, entry: &str) {
        self.entries.borrow_mut().push(entry.to_owned());
    }
}
