use bevy::prelude::*;
use bevy_butler::*;
use bevy_log::info;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Resource)]
#[resource(plugin = MyPlugin, non_send, init = Message("Hello, world!".to_string()))]
struct Message(String);


#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(Startup, |msg: NonSend<Message>| info!("Non-send message: {}", msg.0))
        .run();
}

#[wasm_bindgen_test(unsupported = test)]
#[should_panic]
fn panic_test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(Startup, |msg: Res<Message>| info!("Resource message: {}", msg.0))
        .run();
}