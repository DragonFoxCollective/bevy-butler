use bevy::prelude::*;
use bevy_butler::*;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Resource)]
#[resource(plugin = MyPlugin, init = StartNumber(10))]
struct StartNumber(i32);

fn system1(res: Res<StartNumber>) -> i32 {
    res.0 + 4
}

fn system2(input: In<i32>) -> String {
    input.0.to_string()
}

#[system(plugin = MyPlugin, schedule = Startup, pipe_in(system1, system2))]
fn system3(input: In<String>) {
    info!("Number: {}", *input);
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(Startup, system1.pipe(system2).pipe(system3))
        .run();
}