use bevy_app::{App, Plugin};
use bevy_log::{debug, info, warn};
use bevy_utils::{HashMap, HashSet};
use std::any::{type_name, TypeId};
use std::sync::LazyLock;

pub use bevy_app;
pub use linkme;
use linkme::distributed_slice;

pub type ButlerRegistry = HashMap<TypeId, HashSet<fn(&mut App)>>;

#[distributed_slice]
pub static BUTLER_SLICE: [&'static dyn ButlerSystem];

pub static BUTLER_REGISTRY: LazyLock<ButlerRegistry> = LazyLock::new(|| {
    let mut registry = ButlerRegistry::new();

    let mut sys_count = 0;
    for system in BUTLER_SLICE {
        let (plugin, func) = system.registry_entry();
        let duplicate_system = !registry.entry(plugin).or_default().insert(func);

        assert!(!duplicate_system, "Tried to insert a butler system twice?");
        sys_count += 1;
    }

    info!("Loaded {sys_count} systems for {} plugins", registry.len());
    registry
});

pub trait ButlerPlugin: Plugin {
    fn register_butler_plugins(app: &mut App) {
        match BUTLER_REGISTRY.get(&TypeId::of::<Self>()) {
            None => warn!(
                "Butler plugin {} registered, but no systems registered?",
                type_name::<Self>()
            ),
            Some(funcs) => {
                for func in funcs {
                    (func)(app)
                }

                debug!("{} loaded {} systems", type_name::<Self>(), funcs.len());
            }
        }
    }
}

pub trait ButlerSystem: 'static + Sync + Send {
    fn registry_entry(&self) -> (TypeId, fn(&mut App));
}
