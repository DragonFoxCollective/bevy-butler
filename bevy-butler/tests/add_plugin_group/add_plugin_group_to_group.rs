use bevy::prelude::*;
use bevy_butler::*;

#[butler_plugin_group]
pub struct FooPluginGroup;

#[butler_plugin_group]
#[add_plugin_group(to_group = FooPluginGroup)]
pub struct BarGroup;

#[derive(Resource)]
#[insert_resource(plugin = MarkerPlugin, init = Marker)]
pub struct Marker;

#[butler_plugin]
#[add_plugin(to_group = BarGroup)]
pub struct MarkerPlugin;

#[test]
fn test() {
    App::new()
        .add_plugins(FooPluginGroup)
        .add_systems(Startup, |_: Res<Marker>| ())
        .run();
}
