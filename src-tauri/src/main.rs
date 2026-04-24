// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, OnceLock};

type BackendRunner = fn();

const DEFAULT_BACKEND_RUNNER: BackendRunner = sql_intelliscan_lib::run;

fn backend_runner() -> &'static Mutex<BackendRunner> {
    static BACKEND_RUNNER: OnceLock<Mutex<BackendRunner>> = OnceLock::new();

    BACKEND_RUNNER.get_or_init(|| Mutex::new(DEFAULT_BACKEND_RUNNER))
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

fn main() {
    start_application()
}
