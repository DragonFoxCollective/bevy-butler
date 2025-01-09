#![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]
#![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]

use bevy::MinimalPlugins;
use bevy_app::{App, AppExit, Plugin, Startup, Update};
use bevy_ecs::{event::EventWriter, schedule::IntoSystemConfigs, system::{Res, ResMut, Resource}};
use bevy_butler::*;

#[test]
pub fn test() {
    #[derive(Resource)]
    pub struct Marker(pub bool);

    #[derive(Debug)]
    pub struct TestPlugin;

    #[butler_plugin]
    impl Plugin for TestPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Marker(false));
        }
    }

    #[butler_plugin]
    #[derive(Debug)]
    pub struct OtherTestPlugin;

    #[system(schedule = Startup, plugin = TestPlugin, run_if = || true)]
    fn test_system(
        mut marker: ResMut<Marker>,
    ) {
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

#[cfg(feature="nightly")]
#[test]
fn config_systems_mod_test() {
    use bevy::prelude::*;

    struct MyPlugin;

    #[butler_plugin]
    impl Plugin for MyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Marker(false));
        }
    }

    #[derive(Resource)]
    struct Marker(pub bool);

    #[config_systems(plugin = MyPlugin, schedule = Update)]
    {
        #[system(schedule = Startup)]
        fn hello_world()
        {
            info!("Hello, world!");
        }

        #[system]
        fn goodbye_world(
            time: Res<Time>,
            mut marker: ResMut<Marker>,
        ) {
            info!("The time is {}", time.elapsed_secs());
            marker.0 = true;
        }
    }

    App::new()
        .add_plugins((MinimalPlugins, MyPlugin))
        .add_systems(PostUpdate, |marker: Res<Marker>, mut exit: EventWriter<AppExit>| {
            assert!(marker.0, "Other systems failed to run");
            exit.send(AppExit::Success);
        })
        .run();
}