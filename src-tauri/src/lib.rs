use std::sync::{Mutex, OnceLock};

use sql_intelliscan_services::greet as greet_from_services;

type BuilderFactory = fn() -> tauri::Builder<tauri::Wry>;
type Runner = fn(tauri::Builder<tauri::Wry>);
type BackendRunner = fn();

const DEFAULT_BUILDER_FACTORY: BuilderFactory = build_app;
const DEFAULT_RUNNER: Runner = run_builder;
const DEFAULT_BACKEND_RUNNER: BackendRunner = run;

fn run_hooks() -> &'static Mutex<(BuilderFactory, Runner)> {
    static RUN_HOOKS: OnceLock<Mutex<(BuilderFactory, Runner)>> = OnceLock::new();

    RUN_HOOKS.get_or_init(|| Mutex::new((DEFAULT_BUILDER_FACTORY, DEFAULT_RUNNER)))
}

fn backend_runner() -> &'static Mutex<BackendRunner> {
    static BACKEND_RUNNER: OnceLock<Mutex<BackendRunner>> = OnceLock::new();

    BACKEND_RUNNER.get_or_init(|| Mutex::new(DEFAULT_BACKEND_RUNNER))
}

pub fn greet(name: &str) -> String {
    greet_from_services(name)
}

#[tauri::command]
fn greet_command(name: &str) -> String {
    greet(name)
}

pub fn build_app() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet_command])
}

fn run_builder(builder: tauri::Builder<tauri::Wry>) {
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn run_with(
    builder_factory: fn() -> tauri::Builder<tauri::Wry>,
    runner: fn(tauri::Builder<tauri::Wry>),
) {
    runner(builder_factory());
}

pub fn set_run_hooks(builder_factory: BuilderFactory, runner: Runner) {
    *run_hooks().lock().expect("run hooks lock poisoned") = (builder_factory, runner);
}

pub fn reset_run_hooks() {
    set_run_hooks(DEFAULT_BUILDER_FACTORY, DEFAULT_RUNNER);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (builder_factory, runner) = *run_hooks().lock().expect("run hooks lock poisoned");

    run_with(builder_factory, runner);
}

pub fn run_application(run_backend: fn()) {
    run_backend();
}

pub fn set_backend_runner(run_backend: BackendRunner) {
    *backend_runner()
        .lock()
        .expect("backend runner lock poisoned") = run_backend;
}

pub fn reset_backend_runner() {
    set_backend_runner(DEFAULT_BACKEND_RUNNER);
}

pub fn start_application() {
    let run_backend = *backend_runner()
        .lock()
        .expect("backend runner lock poisoned");

    run_application(run_backend);
}
