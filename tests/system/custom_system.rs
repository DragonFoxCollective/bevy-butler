use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use super::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

#[derive(Resource, Default)]
struct MyResource(String);

fn world() -> String {
    "world".to_string()
}

#[custom_system(
    plugin = MyPlugin,
    schedule = Startup,
    transforms = world.pipe(hello),
)]
fn hello(In(world): In<String>, mut marker: ResMut<MyResource>) {
    info!("Hello, {world}!");
    marker.0 = format!("Hello, {world}!");
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .init_resource::<MyResource>()
        .add_systems(PostStartup, |marker: Res<MyResource>| assert!(marker.0 == "Hello, world!"))
        .run();
}
