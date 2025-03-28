//! This test ensures that Expr-style schedules, like OnEnter(MyState::MyVariant), can be used in #[add_system]

use std::time::Duration;

use bevy::{prelude::*, time::TimePlugin};
use bevy_app::ScheduleRunnerPlugin;
use bevy_butler::*;
use bevy_state::{app::StatesPlugin, prelude::*};
use bevy_log::prelude::*;

use crate::common::log_plugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MyState {
    #[default]
    Start,
    Middle,
    End,
}

#[derive(Resource, Default)]
struct Counter(u8);

#[butler_plugin(build = init_resource::<Counter>)]
struct MyPlugin;

#[add_system(plugin = MyPlugin, schedule = Startup)]
fn start_system(mut counter: ResMut<Counter>, mut next_state: ResMut<NextState<MyState>>) {
    info!("State: Start");
    assert_eq!(counter.0, 0);
    counter.0 = 1;
    next_state.set(MyState::Middle);
}

#[add_system(plugin = MyPlugin, schedule = OnEnter(MyState::Middle))]
fn middle_system(mut counter: ResMut<Counter>, mut next_state: ResMut<NextState<MyState>>) {
    info!("State: Middle");
    assert_eq!(counter.0, 1);
    counter.0 = 2;
    next_state.set(MyState::End);
}

#[add_system(plugin = MyPlugin, schedule = OnEnter(MyState::End))]
fn end_system(mut counter: ResMut<Counter>, mut exit: EventWriter<AppExit>) {
    info!("State: End");
    assert_eq!(counter.0, 2);
    counter.0 = 3;
    exit.send(AppExit::Success);
}

#[add_system(plugin = MyPlugin, schedule = Update, run_if = |time: Res<Time>| time.elapsed_secs() > 3.0f32)]
fn timeout_system() {
    panic!("Test timed out");
}

#[test]
fn test() {
    App::new()
        .add_plugins((
            log_plugin(),
            StatesPlugin,
            TimePlugin,
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(0.1)),
        ))
        .add_plugins(MyPlugin)
        .init_state::<MyState>()
        .run();
}
