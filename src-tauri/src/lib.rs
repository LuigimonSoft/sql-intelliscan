mod bootstrap;
mod commands;
mod config;
mod configuration;
mod dependency_wiring;
mod logging;
mod state;

pub use bootstrap::wiring::configured_connection_string_from_env_value;
pub use commands::{
    connection_string_from_request, greet_command, greet_with_state, register_handlers,
    test_connection, test_connection_with_state, CommandErrorResponse, CommandSuccessResponse,
    ConnectionTestRequest, ConnectionTestResponse,
};
pub use config::connection_config_loader::{
    load_connection_config, load_connection_config_from_connection_string,
    load_connection_config_from_env_value, CONNECTION_STRING_ENV_VAR,
};
pub use config::environment::{
    app_environment_from_env_value, current_app_environment, parse_app_environment, AppEnvironment,
    APP_ENVIRONMENT_ENV_VAR,
};
use configuration::run_builder;
pub use configuration::{build_app, try_build_app};
pub use dependency_wiring::{
    build_app_state, greet_user, shared_app_state, validate_sql_server_connection,
};
pub use logging::{
    build_log_filter, build_log_filter_from_env_value,
    build_log_filter_from_env_value_for_environment, default_log_filter_for_environment,
    init_logging, logging_stack_decision, LoggingInitError, DEFAULT_LOG_FILTER, LOG_FILTER_ENV_VAR,
    PROJECT_LOG_FILTER_ENV_VAR,
};
pub use sql_intelliscan_services::errors::ServiceError;
pub use sql_intelliscan_services::models;
use state::{backend_runner, run_hooks, BackendRunner, BuilderFactory, Runner};
pub use state::{AppState, ConnectionServicePort, GreetingServicePort};

const DEFAULT_BUILDER_FACTORY: BuilderFactory = build_app;
const DEFAULT_RUNNER: Runner = run_builder;
const DEFAULT_BACKEND_RUNNER: BackendRunner = run;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupLogEvent {
    ApplicationStartupStarted,
    LoggingInitialized,
    ApplicationStateBuildStarted,
    TauriApplicationStarting,
}

pub fn greet(name: &str) -> Result<String, ServiceError> {
    greet_user(name)
}

pub fn run_with(
    builder_factory: fn() -> tauri::Builder<tauri::Wry>,
    runner: fn(tauri::Builder<tauri::Wry>),
) {
    runner(builder_factory());
}

pub fn set_run_hooks(builder_factory: BuilderFactory, runner: Runner) {
    *run_hooks(DEFAULT_BUILDER_FACTORY, DEFAULT_RUNNER)
        .lock()
        .expect("run hooks lock poisoned") = (builder_factory, runner);
}

pub fn reset_run_hooks() {
    set_run_hooks(DEFAULT_BUILDER_FACTORY, DEFAULT_RUNNER);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(error) = try_run() {
        eprintln!("{error}");
    }
}

pub fn try_run() -> Result<(), LoggingInitError> {
    let (builder_factory, runner) = *run_hooks(DEFAULT_BUILDER_FACTORY, DEFAULT_RUNNER)
        .lock()
        .expect("run hooks lock poisoned");

    run_startup_with(init_logging, builder_factory, runner, log_startup_event)
}

pub fn run_startup_with<LoggingInitializer, BuilderFactory, Runner, StartupLogger>(
    mut logging_initializer: LoggingInitializer,
    builder_factory: BuilderFactory,
    runner: Runner,
    mut startup_logger: StartupLogger,
) -> Result<(), LoggingInitError>
where
    LoggingInitializer: FnMut() -> Result<(), LoggingInitError>,
    BuilderFactory: FnOnce() -> tauri::Builder<tauri::Wry>,
    Runner: FnOnce(tauri::Builder<tauri::Wry>),
    StartupLogger: FnMut(StartupLogEvent),
{
    logging_initializer()?;

    startup_logger(StartupLogEvent::ApplicationStartupStarted);
    startup_logger(StartupLogEvent::LoggingInitialized);
    startup_logger(StartupLogEvent::ApplicationStateBuildStarted);

    let builder = builder_factory();

    startup_logger(StartupLogEvent::TauriApplicationStarting);
    runner(builder);

    Ok(())
}

pub fn log_startup_event(event: StartupLogEvent) {
    match event {
        StartupLogEvent::ApplicationStartupStarted => {
            tracing::info!(
                target: "sql_intelliscan::startup",
                "Application startup started"
            );
        }
        StartupLogEvent::LoggingInitialized => {
            tracing::info!(target: "sql_intelliscan::startup", "Logging initialized");
        }
        StartupLogEvent::ApplicationStateBuildStarted => {
            tracing::info!(
                target: "sql_intelliscan::startup",
                "Application state build started"
            );
        }
        StartupLogEvent::TauriApplicationStarting => {
            tracing::info!(
                target: "sql_intelliscan::startup",
                "Tauri application starting"
            );
        }
    }
}

pub fn run_application(run_backend: fn()) {
    run_backend();
}

pub fn set_backend_runner(run_backend: BackendRunner) {
    *backend_runner(DEFAULT_BACKEND_RUNNER)
        .lock()
        .expect("backend runner lock poisoned") = run_backend;
}

pub fn reset_backend_runner() {
    set_backend_runner(DEFAULT_BACKEND_RUNNER);
}

pub fn start_application() {
    let run_backend = *backend_runner(DEFAULT_BACKEND_RUNNER)
        .lock()
        .expect("backend runner lock poisoned");

    run_application(run_backend);
}
