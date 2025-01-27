use bevy::prelude::*;
use bevy_butler::*;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

#[derive(Event)]
#[event(plugin = MyPlugin, generics = <String>)]
struct MessageReceived<T>(T);

#[butler_plugin]
struct MyPlugin;

#[derive(Resource, Default)]
#[resource(plugin = MyPlugin)]
struct Marker(bool);

#[system(plugin = MyPlugin, schedule = Startup)]
fn send_message(mut message: EventWriter<MessageReceived<String>>) {
    message.send(MessageReceived("Hello, world!".to_string()));
}

#[system(plugin = MyPlugin, schedule = Startup, after = send_message)]
fn receive_message(mut messages: EventReader<MessageReceived<String>>, mut marker: ResMut<Marker>) {
    for message in messages.read() {
        info!("MessageReceived(\"{}\")", message.0);
        marker.0 = true;
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn main() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |marker: Res<Marker>| assert!(marker.0))
        .run();
}
