use bevy_app::App;
use bevy_butler::*;

use crate::common::log_plugin;

#[butler_plugin_group(name = "MyPluginGroup")]
struct MyPluginGroup;

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPluginGroup)
        .run();
}
