use crate::{commands::register_handlers, dependency_wiring::create_app_state};

pub fn build_app() -> tauri::Builder<tauri::Wry> {
    register_handlers(tauri::Builder::default())
        .manage(create_app_state())
        .plugin(tauri_plugin_opener::init())
}

pub fn run_builder(builder: tauri::Builder<tauri::Wry>) {
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
