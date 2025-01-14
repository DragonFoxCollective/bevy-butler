#![cfg_attr(feature = "nightly", feature(used_with_arg))]

use bevy_app::{App, Plugin, PostStartup, Startup};
use bevy_butler::*;
use bevy_ecs::{
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::{Level, LogPlugin};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test(unsupported = test)]
fn system() {
    #[derive(Resource)]
    struct Marker(pub bool);

    #[derive(Debug)]
    struct TestPlugin;

    #[butler_plugin]
    impl Plugin for TestPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Marker(false));
        }
    }

    #[butler_plugin]
    #[derive(Debug)]
    struct OtherTestPlugin;

    #[system(schedule = Startup, plugin = TestPlugin, run_if = || true)]
    fn test_system(mut marker: ResMut<Marker>) {
        println!("HELLO, WORLD!!!!");
        marker.0 = true;
    }

    #[system(schedule = PostStartup, plugin = TestPlugin, after = test_system, run_if = || true)]
    fn assert_sys(marker: Res<Marker>) {
        assert!(marker.0);
    }

    App::new()
        .add_plugins(LogPlugin {filter: "bevy_butler".to_string(), level: Level::TRACE, ..Default::default() })
        .add_plugins((TestPlugin, OtherTestPlugin))
        .run();
}

#[wasm_bindgen_test(unsupported = test)]
fn systems_with_advanced_plugin() {
    #[derive(Resource)]
    struct MarkerOne(pub u8);

    #[derive(Resource)]
    struct MarkerTwo(pub u8);

    struct MyPlugin;

    #[butler_plugin]
    #[build = insert_resource(MarkerOne(1))]
    impl Plugin for MyPlugin {
        fn build(&self, nonstandard_name: &mut App) {
            nonstandard_name.insert_resource(MarkerTwo(2));
        }
    }

    #[system(plugin = MyPlugin, schedule = Startup)]
    fn marker_one(mut marker: ResMut<MarkerOne>) {
        assert_eq!(marker.0, 1);
        marker.0 = 2;
    }

    #[system(schedule = Startup, plugin = MyPlugin)]
    fn marker_two(mut marker: ResMut<MarkerTwo>) {
        assert_eq!(marker.0, 2);
        marker.0 = 4;
    }

    App::new()
        .add_plugins(LogPlugin {filter: "bevy_butler".to_string(), level: Level::TRACE, ..Default::default() })
        .add_plugins(MyPlugin)
        .add_systems(
            PostStartup,
            |marker1: Res<MarkerOne>, marker2: Res<MarkerTwo>| {
                assert_eq!(marker1.0, 2);
                assert_eq!(marker2.0, 4);
            },
        )
        .run();
}
