use bevy_butler::*;
use bevy::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[butler_plugin]
struct PluginFoo;

#[add_to_plugin(plugin = PluginFoo)]
#[butler_plugin_group]
struct MyPluginGroup;

#[butler_plugin]
#[add_to_group(group = MyPluginGroup)]
struct PluginBar;

#[derive(Resource, Default)]
#[add_resource(plugin = PluginBar)]
struct Marker;

#[wasm_bindgen_test(unsupported = test)]
pub fn add_plugin_test() {
    App::new()
        .add_plugins(PluginFoo)
        .add_systems(Startup, |_marker: Res<Marker>| ())
        .run();
}