#![cfg_attr(feature = "nightly", feature(used_with_arg))]

use bevy::MinimalPlugins;
use bevy_app::{App, AppExit, Plugin, PostStartup, Startup, Update};
use bevy_butler::*;
use bevy_ecs::{
    event::EventWriter,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};

#[test]
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

    #[system(schedule = Update, plugin = TestPlugin, after = test_system, run_if = || true)]
    fn assert_sys(marker: Res<Marker>, mut exit: EventWriter<AppExit>) {
        assert!(marker.0);
        exit.send(AppExit::Success);
    }

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins((TestPlugin, OtherTestPlugin))
        .run();
}

#[test]
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
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |marker1: Res<MarkerOne>, marker2: Res<MarkerTwo>| {
            assert_eq!(marker1.0, 2);
            assert_eq!(marker2.0, 4);
        })
        .run();
}