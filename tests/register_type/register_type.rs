use bevy_butler::*;
use bevy::prelude::*;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Reflect)]
#[register_type(plugin = MyPlugin)]
struct DynamicType {
    pub message: String,
}

// TODO: Write an actual test for this

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}