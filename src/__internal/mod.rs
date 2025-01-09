use std::any::TypeId;
use std::sync::LazyLock;
use bevy_app::App;
use bevy_utils::HashMap;
use bevy_log::{debug, info};

pub use inventory;

pub type ButlerRegistry = HashMap<TypeId, Vec<fn(&mut App) -> ()>>;

/// ButlerFuncs take the registry and add their systems to the relevant
/// plugin's Vec
pub struct ButlerFunc(pub fn(&mut ButlerRegistry) -> ());

pub static BUTLER_REGISTRY: LazyLock<ButlerRegistry> = LazyLock::new(|| {
    let mut registry = ButlerRegistry::new();

    let mut sys_count = 0;
    for butler_func in inventory::iter::<ButlerFunc> {
        (butler_func.0)(&mut registry);
        sys_count += 1;
    }

    info!("Loaded {sys_count} systems for {} plugins", registry.len());
    registry
});

pub fn _butler_debug(msg: &str) {
    debug!("{}", msg);
}

inventory::collect!(ButlerFunc);
