// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn run_application(run_backend: fn()) {
    run_backend();
}

fn main() {
    run_application(sql_intelliscan_lib::run)
}
