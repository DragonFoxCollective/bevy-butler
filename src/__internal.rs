use std::{any::{type_name, TypeId}, collections::HashMap, sync::LazyLock};

use bevy_app::{App, Plugin};
#[cfg(not(feature = "inventory"))]
pub use linkme;

pub use bevy_app;
pub use bevy_ecs;

pub struct ButlerRegistryEntryFactory(fn() -> TypeId, fn(&mut bevy_app::App));

pub struct ButlerRegistry(HashMap<TypeId, Vec<fn(&mut App)>>);

impl ButlerRegistry {
    pub(crate) fn get_system_factories<T: 'static>(&'static self, _marker: SealedMarker<T>) -> &'static [fn(&mut bevy_app::App)] {
        self.0.get(&std::any::TypeId::of::<T>()).map(|v| v.as_slice()).unwrap_or_default()
    }
}

#[cfg(not(feature = "inventory"))]
#[linkme::distributed_slice]
pub static BUTLER_SLICE: [ButlerRegistryEntryFactory] = [..];

#[cfg(feature="inventory")]
::inventory::collect!(ButlerRegistryEntryFactory);

pub static BUTLER_REGISTRY: LazyLock<ButlerRegistry> = LazyLock::new(|| {
    #[cfg(not(feature="inventory"))]
    let iter = BUTLER_SLICE.into_iter();
    #[cfg(feature="inventory")]
    let iter = ::inventory::iter::<ButlerRegistryEntryFactory>.into_iter();

    let mut registry: HashMap<TypeId, Vec<fn(&mut App)>> = HashMap::new();
    iter.for_each(|ButlerRegistryEntryFactory(type_factory, sys_factory)| {
        registry.entry(type_factory())
            .or_default()
            .push(*sys_factory);
    });

    ButlerRegistry(registry)
});

pub struct SealedMarker<T>(pub T);
