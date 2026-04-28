use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectionTestResult {
    pub is_valid: bool,
}

impl ConnectionTestResult {
    pub fn valid() -> Self {
        Self { is_valid: true }
    }

    pub fn invalid() -> Self {
        Self { is_valid: false }
    }
}
