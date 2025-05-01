use bevy_butler::*;
use bevy::prelude::*;

#[butler_plugin]
pub struct FooPlugin;

#[butler_plugin_group]
#[add_plugin_group(to_plugin = FooPlugin)]
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
        .add_plugins(FooPlugin)
        .add_systems(Startup, |_: Res<Marker>| ())
        .run();
}
