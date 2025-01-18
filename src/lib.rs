pub mod __internal;

use std::any::type_name;
use std::any::TypeId;

use __internal::BUTLER_REGISTRY;
use bevy_app::App;
use bevy_app::Plugin;

pub use bevy_butler_proc_macro::butler_plugin;

pub use bevy_butler_proc_macro::system;

pub trait ButlerPlugin: Plugin {
    type Marker;

    fn register_butler_systems(app: &mut App, marker: TypeId) {
        let factories = BUTLER_REGISTRY.get_system_factories(marker);
        for system_factory in factories {
            system_factory(app);
        }
        bevy_log::debug!("{} ran {} factories", type_name::<Self>(), factories.len());
    }
}