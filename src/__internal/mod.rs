use bevy_app::{App, Plugin};
use bevy_log::{debug, warn};
use std::any::{type_name, TypeId};
use registry::BUTLER_REGISTRY;

pub mod registry;
pub use bevy_app;

pub trait ButlerPlugin: Plugin {
    type Marker;

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

    /// Used to implement a marker that is only accessible by pub(crate)
    fn _marker() -> Self::Marker;
}

pub trait ButlerSystem
where
    Self: 'static + Sync + Send,
{
    type Plugin: ButlerPlugin;

    fn system(&self) -> fn(&mut App);
}

// dyn-compatible form of ButlerSystem<Plugin>
pub trait ButlerStaticSystem
where
    Self: 'static + Sync + Send,
{
    fn registry_entry(&self) -> (TypeId, fn(&mut App));
}

impl<TSys, TPlugin> ButlerStaticSystem for TSys
where
    TSys: ButlerSystem<Plugin = TPlugin>,
    TPlugin: ButlerPlugin,
{
    fn registry_entry(&self) -> (TypeId, fn(&mut App)) {
        (TypeId::of::<TPlugin>(), self.system())
    }
}
