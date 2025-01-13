#![cfg_attr(feature = "nightly", feature(used_with_arg))]

use bevy::prelude::{Res, Resource};
use bevy_app::{App, Plugin, PostStartup, Startup};
use bevy_butler::*;

#[test]
fn butler_plugin_struct() {
    #[derive(Resource)]
    struct Marker(pub usize);

    #[butler_plugin]
    #[build(insert_resource = Marker(12))]
    struct MyPlugin;

    App::new()
        .add_plugins(MyPlugin)
        .add_systems(Startup, |marker: Res<Marker>| assert_eq!(marker.0, 12))
        .run();
}

#[test]
fn butler_plugin_impl() {
    struct MyPlugin;

    #[derive(Resource)]
    struct Marker(pub &'static str);

    #[butler_plugin]
    impl Plugin for MyPlugin {
        fn build(&self, nonstandard_name: &mut App) {
            nonstandard_name.insert_resource(Marker("MyMarker"));
        }
    }

    let mut app = App::new();
    app.add_plugins(MyPlugin);
    app.add_systems(PostStartup, |marker: Res<Marker>| {
        assert_eq!(marker.0, "MyMarker");
    });
    app.run();
}

#[test]
fn butler_advanced_plugin_impl() {
    struct MyPlugin;

    #[derive(Resource)]
    struct MarkerOne(pub u8);

    #[derive(Resource)]
    struct MarkerTwo(pub u8);

    #[butler_plugin]
    #[build = insert_resource(MarkerOne(1))]
    impl Plugin for MyPlugin {
        fn build(&self, nonstandard_name: &mut App) {
            nonstandard_name.insert_resource(MarkerTwo(2));
        }
    }

    let mut app = App::new();
    app.add_plugins(MyPlugin);
    app.add_systems(
        PostStartup,
        |marker1: Res<MarkerOne>, marker2: Res<MarkerTwo>| {
            assert_eq!(marker1.0, 1);
            assert_eq!(marker2.0, 2);
        },
    );
    app.run();
}

#[test]
fn butler_advanced_plugin_single_attr_impl() {
    struct MyPlugin;

    #[derive(Resource)]
    struct MarkerOne(pub u8);

    #[derive(Resource)]
    struct MarkerTwo(pub u8);

    #[butler_plugin(
        build = insert_resource(MarkerOne(1))
    )]
    impl Plugin for MyPlugin {
        fn build(&self, nonstandard_name: &mut App) {
            nonstandard_name.insert_resource(MarkerTwo(2));
        }
    }

    let mut app = App::new();
    app.add_plugins(MyPlugin);
    app.add_systems(
        PostStartup,
        |marker1: Res<MarkerOne>, marker2: Res<MarkerTwo>| {
            assert_eq!(marker1.0, 1);
            assert_eq!(marker2.0, 2);
        },
    );
    app.run();
}
