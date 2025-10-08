//! Test that unit plugins can be added without an `init` and
//! without a `Default` implementation
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Resource)]
pub struct Marker;

#[add_plugin(to_plugin = FooPlugin)]
pub struct BarPlugin;

#[butler_plugin]
impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Marker);
    }
}

#[butler_plugin]
pub struct FooPlugin;

#[test]
fn test() {
    App::new()
        .add_plugins(FooPlugin)
        .add_systems(Startup, |_: Res<Marker>| ())
        .run();
}
