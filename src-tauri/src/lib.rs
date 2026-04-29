mod commands;
mod configuration;
mod dependency_wiring;
mod state;

pub use commands::{
    greet_command_handler, register_handlers, validate_sql_server_connection_command,
};
pub use configuration::build_app;
use configuration::run_builder;
pub use dependency_wiring::{create_app_state, validate_sql_server_connection};
pub use state::AppState;
use state::{backend_runner, run_hooks, BackendRunner, BuilderFactory, Runner};

const DEFAULT_BUILDER_FACTORY: BuilderFactory = build_app;
const DEFAULT_RUNNER: Runner = run_builder;
const DEFAULT_BACKEND_RUNNER: BackendRunner = run;

pub fn greet(name: &str) -> String {
    let state = create_app_state();

    dependency_wiring::greet_user(name, &state)
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
    let (builder_factory, runner) = *run_hooks(DEFAULT_BUILDER_FACTORY, DEFAULT_RUNNER)
        .lock()
        .expect("run hooks lock poisoned");

    run_with(builder_factory, runner);
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
