use bevy_app::{Plugin, App};
use bevy_utils::HashMap;
use bevy_log::info;
use std::any::TypeId;
use crate::__internal::*;

pub struct BevyButlerPlugin;

impl Plugin for BevyButlerPlugin {
    fn build(&self, app: &mut App) {
        let mut registry: HashMap<TypeId, Vec<&'static ButlerFunc>> = HashMap::new();

        let mut global_systems = 0;
        let mut plugin_systems = 0;
        for system in inventory::iter::<ButlerFunc> {
            if let Some(global_sys) = system.try_get_func::<Self>() {
                (global_sys)(self, app);
                global_systems += 1;
            }
            else {
                registry.entry(system.type_id())
                    .or_default()
                    .push(system);
                plugin_systems += 1;
            }
        }

        app.insert_resource(ButlerRegistry::new(registry));

        info!("{} total butler systems loaded ({global_systems} global, {plugin_systems} plugin)", global_systems + plugin_systems);
    }
}