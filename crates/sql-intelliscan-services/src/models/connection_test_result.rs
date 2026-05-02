use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectionTestResult {
    pub is_valid: bool,
    pub message: String,
    pub database: Option<String>,
    pub latency_ms: Option<u64>,
}

impl ConnectionTestResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            message: "Connection successful".to_owned(),
            database: None,
            latency_ms: None,
        }
    }

    pub fn valid_with_details(database: Option<String>, latency_ms: Option<u64>) -> Self {
        Self {
            database,
            latency_ms,
            ..Self::valid()
        }
    }

    pub fn invalid() -> Self {
        Self {
            is_valid: false,
            message: "Connection failed".to_owned(),
            database: None,
            latency_ms: None,
        }
    }
}
