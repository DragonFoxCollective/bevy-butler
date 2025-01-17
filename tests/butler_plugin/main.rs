use bevy_butler::*;
use bevy_app::prelude::*;

#[butler_plugin(build(a))]
struct MyPlugin;

#[test]
pub fn butler_plugin_test() {
    App::new()
        .add_plugins(MyPlugin)
        .run();
}
