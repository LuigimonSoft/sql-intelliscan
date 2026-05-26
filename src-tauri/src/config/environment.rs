use std::env::VarError;

pub const APP_ENVIRONMENT_ENV_VAR: &str = "SQL_INTELLISCAN_ENV";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnvironment {
    Development,
    Test,
    Staging,
    Production,
}

impl AppEnvironment {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Test => "test",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

pub fn app_environment_from_env_value(
    configured_environment: Result<String, VarError>,
) -> AppEnvironment {
    match configured_environment {
        Ok(environment) => {
            parse_app_environment(&environment).unwrap_or(AppEnvironment::Production)
        }
        Err(VarError::NotPresent) => AppEnvironment::Development,
        Err(VarError::NotUnicode(_)) => AppEnvironment::Production,
    }
}

pub fn current_app_environment() -> AppEnvironment {
    app_environment_from_env_value(std::env::var(APP_ENVIRONMENT_ENV_VAR))
}

pub fn parse_app_environment(environment: &str) -> Option<AppEnvironment> {
    match environment.trim().to_ascii_lowercase().as_str() {
        "development" | "dev" => Some(AppEnvironment::Development),
        "test" | "testing" => Some(AppEnvironment::Test),
        "staging" | "stage" => Some(AppEnvironment::Staging),
        "production" | "prod" => Some(AppEnvironment::Production),
        _ => None,
    }
}
