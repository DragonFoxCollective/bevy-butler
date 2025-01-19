use bevy_app::prelude::*;
use bevy_butler::*;

struct MyPlugin;

#[butler_plugin]
impl Plugin for MyPlugin {}

#[test]
fn test() {
    App::new()
        .add_plugins(MyPlugin)
        .run();
}