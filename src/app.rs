#[cfg(not(target_arch = "wasm32"))]
pub use crate::components::app::spawn_greet;
pub use crate::components::app::App;
pub use crate::services::greeting_service::{
    greet_message, greet_message_sync, invoke_greet_sync, should_send_greet,
};
