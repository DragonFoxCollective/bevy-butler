#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
pub use inventory;
#[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
pub use linkme;

pub use bevy_app;
pub use bevy_ecs;
pub use bevy_log;
pub use bevy_state;

mod plugin;
pub use plugin::*;

mod plugin_group;
pub use plugin_group::*;
