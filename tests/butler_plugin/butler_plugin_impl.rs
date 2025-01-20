use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_butler::*;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

struct MyPlugin;

#[derive(Resource, Default)]
struct MarkerOne;

#[derive(Resource, Default)]
struct MarkerTwo;

#[butler_plugin(build = init_resource::<MarkerOne>)]
impl Plugin for MyPlugin {
    fn build(&self, nonstandard_app_name: &mut App) {
        nonstandard_app_name.init_resource::<MarkerTwo>();
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(Startup, (
            |_res: Res<MarkerOne>| (),
            |_res: Res<MarkerTwo>| (),
        ))
        .run();
}
