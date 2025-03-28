use bevy::prelude::*;
use bevy_butler::*;
use bevy_log::prelude::*;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Reflect, Resource)]
#[register_type(plugin = MyPlugin)]
struct DynamicMessage {
    pub message: String,
}

#[add_system(plugin = MyPlugin, schedule = Startup)]
fn read_type_registration(registry: Res<AppTypeRegistry>) {
    let registry = registry.read();
    let type_data = registry
        .get_with_short_type_path("DynamicMessage")
        .expect("DynamicMessage was not registered to the type registry");

    info!("Type registration for DynamicMessage: {type_data:?}");
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}
