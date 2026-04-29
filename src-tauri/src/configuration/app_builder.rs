use crate::commands::register_handlers;
use crate::dependency_wiring::build_app_state;
use crate::state::AppStateResult;

pub fn try_build_app() -> AppStateResult<tauri::Builder<tauri::Wry>> {
    let app_state = build_app_state()?;

    Ok(register_handlers(tauri::Builder::default())
        .manage(app_state)
        .plugin(tauri_plugin_opener::init()))
}

pub fn build_app() -> tauri::Builder<tauri::Wry> {
    try_build_app().expect("application dependencies should be valid")
}

pub fn run_builder(builder: tauri::Builder<tauri::Wry>) {
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
