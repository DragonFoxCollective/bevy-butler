use std::fmt::Display;

use bevy::prelude::*;
use bevy_butler::*;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Resource)]
#[insert_resource(plugin = MyPlugin, init = GenericRes(5u8))]
struct GenericRes<T>(T);

fn generic_pipe<T: 'static + Sync + Send + Display>(res: Res<GenericRes<T>>) -> String {
    res.0.to_string()
}

#[add_system(plugin = MyPlugin, schedule = Startup, pipe_in = [generic_pipe::<u8>])]
fn print_res(input: In<String>) {
    info!("Generic resource: {}", input.0);
    assert_eq!(input.0, "5");
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}
