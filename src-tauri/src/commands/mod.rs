use crate::dependency_wiring::greet_user;

#[tauri::command]
pub fn greet_command(name: &str) -> String {
    greet_user(name)
}

pub fn register_handlers(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![greet_command])
}
