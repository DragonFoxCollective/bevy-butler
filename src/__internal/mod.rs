use std::any::TypeId;
use std::sync::LazyLock;
use bevy_app::App;
use bevy_utils::HashMap;
use bevy_log::{debug, info};

pub use linkme;
use linkme::distributed_slice;

pub type ButlerRegistry = HashMap<TypeId, Vec<fn(&mut App) -> ()>>;

/// ButlerFuncs take the registry and add their systems to the relevant
/// plugin's Vec
pub type ButlerFunc = fn(&mut ButlerRegistry) -> ();

#[distributed_slice]
pub static BUTLER_SLICE: [ButlerFunc];

pub static BUTLER_REGISTRY: LazyLock<ButlerRegistry> = LazyLock::new(|| {
    let mut registry = ButlerRegistry::new();

    let mut sys_count = 0;
    for butler_func in BUTLER_SLICE {
        (butler_func)(&mut registry);
        sys_count += 1;
    }

    info!("Loaded {sys_count} systems for {} plugins", registry.len());
    registry
});

pub fn _butler_debug(msg: &str) {
    debug!(target: "bevy-butler", "{}", msg);
}
