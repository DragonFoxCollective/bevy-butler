use bevy_app::{App, Startup};
use bevy_butler::*;
use bevy_log::info;

use crate::common::log_plugin;

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

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins((PluginOne, PluginTwo))
        .run();
}