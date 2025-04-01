use bevy::prelude::*;
use bevy_butler::*;

use crate::common::log_plugin;

#[allow(dead_code)]
#[butler_plugin]
enum MyPlugin {
    VariantOne,
    VariantTwo,
}

#[derive(Resource)]
struct SuccessMarker;

#[add_system(plugin = MyPlugin, schedule = Startup)]
fn hello_world(mut commands: Commands) {
    info!("Hello, world!");
    commands.insert_resource(SuccessMarker);
}

#[add_system(plugin = MyPlugin, schedule = Startup, after = hello_world)]
fn assert_marker(_res: Res<SuccessMarker>) {
    // This will panic if SuccessMarker isn't inserted, so we don't actually
    // need to assert
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin::VariantTwo)
        .run();
}