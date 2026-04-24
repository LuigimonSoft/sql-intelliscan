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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_with(build_app, run_builder);
}
