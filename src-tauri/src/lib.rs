use std::sync::{Mutex, OnceLock};

type BuilderFactory = fn() -> tauri::Builder<tauri::Wry>;
type Runner = fn(tauri::Builder<tauri::Wry>);

const DEFAULT_BUILDER_FACTORY: BuilderFactory = build_app;
const DEFAULT_RUNNER: Runner = run_builder;

fn run_hooks() -> &'static Mutex<(BuilderFactory, Runner)> {
    static RUN_HOOKS: OnceLock<Mutex<(BuilderFactory, Runner)>> = OnceLock::new();

    RUN_HOOKS.get_or_init(|| Mutex::new((DEFAULT_BUILDER_FACTORY, DEFAULT_RUNNER)))
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
