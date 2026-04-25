#[cfg(target_arch = "wasm32")]
#[path = "frontend/app_test.rs"]
mod app;


#[cfg(not(target_arch = "wasm32"))]
#[path = "frontend/native_app_test.rs"]
mod native_app_test;
