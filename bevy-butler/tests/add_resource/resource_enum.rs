use bevy::prelude::*;
use bevy_butler::*;
use wasm_bindgen_test::wasm_bindgen_test;
use bevy_log::prelude::*;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[allow(dead_code)]
#[derive(Resource, Default, Debug)]
#[add_resource(plugin = MyPlugin)]
enum Message {
    Variant1,
    #[default]
    Variant2
}

#[add_system(plugin = MyPlugin, schedule = Startup)]
fn print_resource(res: Res<Message>) {
    info!("Resource: {res:?}");
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}