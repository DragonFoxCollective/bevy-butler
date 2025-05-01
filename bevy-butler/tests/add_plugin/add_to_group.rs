use bevy::prelude::*;
use bevy_butler::*;

#[butler_plugin_group]
pub struct FooGroup;

#[butler_plugin]
#[add_plugin(to_group = FooGroup)]
pub struct Bar;

#[derive(Resource, Default)]
#[insert_resource(plugin = Bar)]
pub struct Marker;

#[test]
fn test() {
    App::new()
        .add_plugins(FooGroup)
        .add_systems(Startup, |_: Res<Marker>| ())
        .run();
}
