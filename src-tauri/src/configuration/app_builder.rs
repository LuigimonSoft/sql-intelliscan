use crate::commands::register_handlers;

pub fn build_app() -> tauri::Builder<tauri::Wry> {
    register_handlers(tauri::Builder::default()).plugin(tauri_plugin_opener::init())
}

pub fn run_builder(builder: tauri::Builder<tauri::Wry>) {
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
