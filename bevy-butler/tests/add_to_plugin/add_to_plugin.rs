use bevy_app::{App, Startup};
use bevy_butler::*;
use bevy_ecs::system::{Res, Resource};
use wasm_bindgen_test::wasm_bindgen_test;

#[derive(Resource, Default)]
#[resource(plugin = PluginBar)]
struct Marker;

#[butler_plugin]
struct PluginFoo;

#[butler_plugin]
#[add_to_plugin(plugin = PluginFoo)]
struct PluginBar;

#[wasm_bindgen_test(unsupported = test)]
pub fn add_plugin_test() {
    App::new()
        .add_plugins(PluginFoo)
        .add_systems(Startup, |_marker: Res<Marker>| ())
        .run();
}