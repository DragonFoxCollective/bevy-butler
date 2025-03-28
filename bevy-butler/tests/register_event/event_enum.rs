use bevy::prelude::*;
use bevy_butler::*;
use bevy_log::prelude::*;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Event, Debug)]
#[register_event(plugin = MyPlugin)]
enum Message {
    Hello(String),
    Goodbye(String),
}

#[derive(Resource)]
struct HelloReceived;
#[derive(Resource)]
struct GoodbyeReceived;

#[add_observer(plugin = MyPlugin)]
fn received_message(
    message: Trigger<Message>,
    mut commands: Commands
) {
    match &*message {
        Message::Hello(name) => {
            info!("Hello, {name}!");
            commands.insert_resource(HelloReceived);
        }
        Message::Goodbye(name) => {
            info!("Goodbye, {name}!");
            commands.insert_resource(GoodbyeReceived);
        }
    }
}

#[add_system(plugin = MyPlugin, schedule = Startup)]
fn send_messages(mut commands: Commands) {
    commands.trigger(Message::Hello("World".to_string()));
    commands.trigger(Message::Goodbye("World".to_string()));
}

#[add_system(plugin = MyPlugin, schedule = Update, run_if = |time: Res<Time>| time.elapsed_secs() > 3f32)]
fn timeout() {
    panic!("Test timed out");
}

#[add_system(plugin = MyPlugin, schedule = Update)]
fn exit_if_messages_received(
    hello: Option<Res<HelloReceived>>,
    goodbye: Option<Res<GoodbyeReceived>>,
    mut exit: EventWriter<AppExit>,
) {
    if hello.is_some() && goodbye.is_some() {
        exit.write(AppExit::Success);
    }
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MinimalPlugins)
        .add_plugins(MyPlugin)
        .run();
}