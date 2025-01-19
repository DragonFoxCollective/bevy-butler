use bevy_app::prelude::*;
use bevy_butler::*;
use wasm_bindgen_test::wasm_bindgen_test;

struct MyPlugin;

#[butler_plugin]
impl Plugin for MyPlugin {}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new().add_plugins(MyPlugin).run();
}
