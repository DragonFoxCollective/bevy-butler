use std::{any::{type_name, TypeId}, collections::HashMap, sync::LazyLock};

use bevy_app::PluginGroupBuilder;

type PluginGroupStep = fn(PluginGroupBuilder) -> PluginGroupBuilder;

pub struct ButlerPluginGroupRegistryEntryFactory {
    pub type_factory: fn() -> TypeId,
    pub group_factory: PluginGroupStep,
}

pub struct ButlerPluginGroupRegistry(HashMap<TypeId, Vec<PluginGroupStep>>);

impl ButlerPluginGroupRegistry {
    pub(crate) fn get_factories(
        &'static self,
        marker: TypeId,
    ) -> &'static [PluginGroupStep] {
        self.0
            .get(&marker)
            .map(|v| v.as_slice())
            .unwrap_or_default()
    }
}

#[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
#[linkme::distributed_slice]
pub static BUTLER_PLUGIN_GROUP_SLICE: [ButlerPluginGroupRegistryEntryFactory] = [..];

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
::inventory::collect!(ButlerPluginGroupRegistryEntryFactory);

pub static BUTLER_PLUGIN_GROUP_REGISTRY: LazyLock<ButlerPluginGroupRegistry> = LazyLock::new(|| {
    #[cfg(target_arch = "wasm32")]
    crate::_initialize();

    #[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
    let iter = BUTLER_PLUGIN_GROUP_SLICE.into_iter();
    #[cfg(any(target_arch = "wasm32", feature = "inventory"))]
    let iter = ::inventory::iter::<ButlerPluginGroupRegistryEntryFactory>.into_iter();

    let mut count = 0;
    let mut registry: HashMap<TypeId, Vec<PluginGroupStep>> = HashMap::new();
    iter.for_each(|factory| {
        registry
            .entry((factory.type_factory)())
            .or_default()
            .push(factory.group_factory);
        count += 1;
    });

    // Trim down
    registry.values_mut().for_each(|vec| vec.shrink_to_fit());
    registry.shrink_to_fit();

    bevy_log::debug!("Building ButlerPluginGroupRegistry from {count} entries");

    ButlerPluginGroupRegistry(registry)
});

pub trait ButlerPluginGroup {
    fn register_plugins(mut builder: PluginGroupBuilder, marker: TypeId) -> PluginGroupBuilder {
        let factories = BUTLER_PLUGIN_GROUP_REGISTRY.get_factories(marker);
        for plugin_factory in factories {
            builder = plugin_factory(builder);
        }
        bevy_log::debug!("{} ran {} factories", type_name::<Self>(), factories.len());
        builder
    }
}

#[cfg(not(any(target_arch = "wasm32", feature = "inventory")))]
#[macro_export]
#[doc(hidden)]
macro_rules! _butler_plugin_group_entry {
    ($static_ident:ident, $entry:expr) => {
        #[::bevy_butler::__internal::linkme::distributed_slice(
            ::bevy_butler::__internal::BUTLER_PLUGIN_GROUP_SLICE
        )]
        #[linkme(crate = ::bevy_butler::__internal::linkme)]
        #[allow(non_upper_case_globals)]
        static $static_ident: ::bevy_butler::__internal::ButlerPluginGroupRegistryEntryFactory = $entry;
    };
}

#[cfg(any(target_arch = "wasm32", feature = "inventory"))]
#[macro_export]
#[doc(hidden)]
macro_rules! _butler_plugin_group_entry {
    ($static_ident:ident, $entry:expr) => {
        ::bevy_butler::__internal::inventory::submit!($entry);
    };
}
