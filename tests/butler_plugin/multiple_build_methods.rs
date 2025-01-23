use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use super::common::log_plugin;

#[derive(Resource, Default)]
struct CounterOne(pub u8);

#[derive(Resource, Default)]
struct CounterTwo(pub u8);

#[butler_plugin(build(init_resource::<CounterOne>(), init_resource::<CounterTwo>()))]
struct MyPlugin;

#[wasm_bindgen_test(unsupported = test)]
pub fn butler_plugin_test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(Startup, |counter: Res<CounterOne>| assert_eq!(counter.0, 0))
        .add_systems(Startup, |counter: Res<CounterTwo>| assert_eq!(counter.0, 0))
        .run();
}
