use bevy::prelude::*;
use bevy_butler::*;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

#[derive(Message, Debug)]
#[add_message(plugin = MyPlugin)]
enum MessageReceived {
    Hello(String),
    Goodbye(String),
}

#[butler_plugin]
struct MyPlugin;

#[derive(Resource, Default)]
#[insert_resource(plugin = MyPlugin)]
struct Marker {
    hello: bool,
    goodbye: bool,
}

#[add_system(plugin = MyPlugin, schedule = Startup)]
fn send_messages(mut messages: MessageWriter<MessageReceived>) {
    messages.write(MessageReceived::Hello("World".to_string()));
    messages.write(MessageReceived::Goodbye("World".to_string()));
}

#[add_system(plugin = MyPlugin, schedule = Startup, after = send_messages)]
fn received_message(mut messages: MessageReader<MessageReceived>, mut marker: ResMut<Marker>) {
    for message in messages.read() {
        match message {
            MessageReceived::Hello(name) => {
                info!("Hello, {name}!");
                marker.hello = true;
            }
            MessageReceived::Goodbye(name) => {
                info!("Goodbye, {name}!");
                marker.goodbye = true;
            }
        }
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn main() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, (
            |marker: Res<Marker>| assert!(marker.hello),
            |marker: Res<Marker>| assert!(marker.goodbye),
        ))
        .run();
}
