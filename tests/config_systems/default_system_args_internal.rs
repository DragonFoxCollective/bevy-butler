use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

#[butler_plugin(build = init_resource::<Counter>)]
struct MyPlugin;

#[butler_plugin]
struct OtherPlugin;

#[derive(Resource, Default)]
#[::bevy_butler::_butler_config_systems_defaults(plugin = MyPlugin, schedule = Startup)] // Should do nothing on non-systems
struct Counter(pub u8);

#[system]
#[::bevy_butler::_butler_config_systems_defaults(plugin = MyPlugin, schedule = Startup)]
fn system_one(mut res: ResMut<Counter>) {
    info!("System one!");
    res.0 += 1;
}

#[system]
#[system(plugin = OtherPlugin)]
#[_butler_config_systems_defaults(plugin = MyPlugin, schedule = Startup)]
fn system_two(mut res: ResMut<Counter>) {
    info!("System two!");
    res.0 += 1;
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins((MyPlugin, OtherPlugin))
        .add_systems(PostStartup, |res: Res<Counter>| assert_eq!(res.0, 3))
        .run();
}
