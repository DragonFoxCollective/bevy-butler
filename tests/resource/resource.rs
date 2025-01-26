use bevy::prelude::{Res, ResMut, Resource};
use bevy_app::{App, PostStartup, Startup};
use bevy_butler::*;
use bevy_log::info;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

#[derive(Resource, Default)]
#[resource(plugin = MyPlugin)]
struct Marker(bool);

#[derive(Resource)]
#[resource(plugin = MyPlugin, init = Message("Hello, world!".to_string()))]
struct Message(String);

#[butler_plugin]
struct MyPlugin;

#[system(plugin = MyPlugin, schedule = Startup)]
fn get_and_print_message(
    message: Res<Message>,
    mut marker: ResMut<Marker>,
) {
    info!("Resource message: {}", message.0);
    marker.0 = true;
}

#[system(plugin = MyPlugin, schedule = PostStartup)]
fn assert_marker(marker: Res<Marker>) {
    assert!(marker.0);
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}