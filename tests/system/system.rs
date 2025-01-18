use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_butler::*;

#[butler_plugin]
struct MyPlugin;

#[derive(Resource, Default)]
struct Marker(bool);

#[system(
    generics = <TypeA, TypeB>,
    plugin = MyPlugin,
    schedule = Startup,
)]
fn hello_world(mut marker: ResMut<Marker>) {
    info!("Hello, world!");
    marker.0 = true;
}

#[test]
fn test() {
    App::new()
        .init_resource::<Marker>()
        .add_systems(PostStartup, |marker: Res<Marker>| assert!(marker.0))
        .run();
}