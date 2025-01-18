use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_butler::*;

use super::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Resource, Default)]
struct Marker(bool);

#[system(
    plugin = MyPlugin,
    schedule = Startup,
)]
fn hello_world(mut marker: ResMut<Marker>) {
    info!("Hello, world!");
    marker.0 = true;
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .init_resource::<Marker>()
        .add_systems(PostStartup, |marker: Res<Marker>| assert!(marker.0))
        .run();
}