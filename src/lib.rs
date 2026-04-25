#[cfg(target_arch = "wasm32")]
#[path = "app_wasm.rs"]
pub mod app;

#[cfg(not(target_arch = "wasm32"))]
#[path = "app.rs"]
pub mod app;

pub use app::App;
