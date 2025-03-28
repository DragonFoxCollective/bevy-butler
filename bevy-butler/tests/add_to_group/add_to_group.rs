use bevy::prelude::*;
use bevy_app::{App, Startup};
use bevy_butler::*;
use bevy_log::info;

use crate::common::log_plugin;

#[derive(Resource)]
struct Marker;

#[butler_plugin_group]
struct MyPluginGroup;

#[butler_plugin]
#[add_to_group(group = MyPluginGroup)]
struct MyPlugin;

#[add_system(plugin = MyPlugin, schedule = Startup)]
fn hello_world(mut commands: Commands) {
    info!("Hello, world!");
    commands.insert_resource(Marker);
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPluginGroup)
        .add_systems(PostStartup, |_: Res<Marker>| ())
        .run();
}