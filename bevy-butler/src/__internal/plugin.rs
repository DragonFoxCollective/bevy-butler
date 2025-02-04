use bevy_app::{App, Plugin};
use std::{
    any::{type_name, TypeId},
    collections::HashMap,
    sync::LazyLock,
};

pub struct ButlerPluginRegistryEntryFactory(fn() -> TypeId, fn(&mut bevy_app::App));

impl ButlerPluginRegistryEntryFactory {
    pub const fn new(type_factory: fn() -> TypeId, sys_factory: fn(&mut bevy_app::App)) -> Self {
        ButlerPluginRegistryEntryFactory(type_factory, sys_factory)
    }
}

pub struct ButlerPluginRegistry(HashMap<TypeId, Vec<fn(&mut App)>>);

impl ButlerPluginRegistry {
    pub(crate) fn get_system_factories(
        &'static self,
        marker: TypeId,
    ) -> &'static [fn(&mut bevy_app::App)] {
        self.0
            .get(&marker)
            .map(|v| v.as_slice())
            .unwrap_or_default()
    }
}

#[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
#[linkme::distributed_slice]
pub static BUTLER_SLICE: [ButlerPluginRegistryEntryFactory] = [..];

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
::inventory::collect!(ButlerPluginRegistryEntryFactory);

pub static BUTLER_PLUGIN_REGISTRY: LazyLock<ButlerPluginRegistry> = LazyLock::new(|| {
    #[cfg(target_arch = "wasm32")]
    crate::_initialize();

    #[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
    let iter = BUTLER_SLICE.into_iter();
    #[cfg(any(target_arch = "wasm32", feature = "inventory"))]
    let iter = ::inventory::iter::<ButlerPluginRegistryEntryFactory>.into_iter();

    let mut count = 0;
    let mut registry: HashMap<TypeId, Vec<fn(&mut App)>> = HashMap::new();
    iter.for_each(|ButlerPluginRegistryEntryFactory(type_factory, sys_factory)| {
        registry
            .entry(type_factory())
            .or_default()
            .push(*sys_factory);
        count += 1;
    });

    // Trim down
    registry.values_mut().for_each(|vec| vec.shrink_to_fit());
    registry.shrink_to_fit();

    bevy_log::debug!("Building ButlerRegistry from {count} entries");

    ButlerPluginRegistry(registry)
});

pub trait ButlerPlugin: Plugin {
    fn register_butler_systems(app: &mut App, marker: TypeId) {
        let factories = BUTLER_PLUGIN_REGISTRY.get_system_factories(marker);
        for system_factory in factories {
            system_factory(app);
        }
        bevy_log::debug!("{} ran {} factories", type_name::<Self>(), factories.len());
    }
}

#[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
#[macro_export]
#[doc(hidden)]
macro_rules! _butler_plugin_entry {
    ($static_ident:ident, $entry:expr) => {
        #[::bevy_butler::__internal::linkme::distributed_slice(
            ::bevy_butler::__internal::BUTLER_SLICE
        )]
        #[linkme(crate = ::bevy_butler::__internal::linkme)]
        static $static_ident: ::bevy_butler::__internal::ButlerPluginRegistryEntryFactory = $entry;
    };
}

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
#[macro_export]
#[doc(hidden)]
macro_rules! _butler_plugin_entry {
    ($static_ident:ident, $entry:expr) => {
        ::bevy_butler::__internal::inventory::submit!($entry);
    };
}
