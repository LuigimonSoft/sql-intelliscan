#![cfg(not(target_arch = "wasm32"))]

#[path = "backend/lib/mod.rs"]
mod lib;

#[path = "backend/main/mod.rs"]
mod main;
