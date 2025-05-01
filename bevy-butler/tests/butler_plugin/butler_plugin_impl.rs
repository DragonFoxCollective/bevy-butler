use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

struct MyPlugin;

#[derive(Resource, Default)]
struct Marker;

#[butler_plugin]
impl Plugin for MyPlugin {
    fn build(&self, nonstandard_app_name: &mut App) {
        nonstandard_app_name.init_resource::<Marker>();
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(
            Startup,
            |_res: Res<Marker>| (),
        )
        .run();
}
