use linkme::distributed_slice;

use crate::__internal::{registry::ButlerRegistry, ButlerStaticSystem};

pub use ::linkme;

#[distributed_slice]
pub static BUTLER_SLICE: [&'static dyn ButlerStaticSystem];

pub(super) const CONSTRUCT_BUTLER_REGISTRY: fn() -> ButlerRegistry = || {
    let mut registry = ButlerRegistry::new();

    let mut sys_count = 0;
    for system in BUTLER_SLICE {
        let (plugin, func) = system.registry_entry();
        let duplicate_system = !registry.entry(plugin).or_default().insert(func);

        assert!(!duplicate_system, "Tried to insert a butler system twice?");
        sys_count += 1;
    }

    bevy_log::info!("Loaded {sys_count} systems for {} plugins", registry.len());
    registry
};

#[macro_export]
macro_rules! __register_system {
    ($static_name:ident, $sys_struct:expr) => {
        #[::bevy_butler::__internal::registry::linkme::distributed_slice(::bevy_butler::__internal::registry::BUTLER_SLICE)]
        #[linkme(crate = ::bevy_butler::__internal::registry::linkme)] // I LOVE UNDOCUMENTED ATTRIBUTES!!! FUCK!!!
        #[allow(non_upper_case_globals)]
        static $static_name: &'static dyn ::bevy_butler::__internal::ButlerStaticSystem = & $sys_struct;
    };
}
