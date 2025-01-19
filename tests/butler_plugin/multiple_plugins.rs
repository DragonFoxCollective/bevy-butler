use bevy_app::{App, Startup};
use bevy_butler::*;
use bevy_log::info;

use crate::common::log_plugin;
use wasm_bindgen_test::wasm_bindgen_test;

#[butler_plugin]
struct PluginOne;

#[butler_plugin]
struct PluginTwo;

#[system(plugin = PluginOne, schedule = Startup)]
fn system_one() {
    info!("System one!");
}

#[system(plugin = PluginTwo, schedule = Startup)]
fn system_two() {
    info!("System two!");
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins((PluginOne, PluginTwo))
        .run();
}
