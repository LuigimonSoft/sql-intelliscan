use std::sync::{Mutex, OnceLock};

pub type BuilderFactory = fn() -> tauri::Builder<tauri::Wry>;
pub type Runner = fn(tauri::Builder<tauri::Wry>);
pub type BackendRunner = fn();

pub fn run_hooks(
    default_builder_factory: BuilderFactory,
    default_runner: Runner,
) -> &'static Mutex<(BuilderFactory, Runner)> {
    static RUN_HOOKS: OnceLock<Mutex<(BuilderFactory, Runner)>> = OnceLock::new();

    RUN_HOOKS.get_or_init(|| Mutex::new((default_builder_factory, default_runner)))
}

pub fn backend_runner(default_backend_runner: BackendRunner) -> &'static Mutex<BackendRunner> {
    static BACKEND_RUNNER: OnceLock<Mutex<BackendRunner>> = OnceLock::new();

    BACKEND_RUNNER.get_or_init(|| Mutex::new(default_backend_runner))
}
