#![cfg_attr(feature="nightly", feature(stmt_expr_attributes))]
#![cfg_attr(feature="nightly", feature(proc_macro_hygiene))]
#![cfg_attr(feature="nightly", feature(used_with_arg))]

use bevy_ecs::system::Resource;
use bevy_butler::*;

#[cfg(feature="nightly")]
#[test]
fn config_systems_attr_test() {
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

    #[config_systems_block(plugin = MyPlugin, schedule = Update)]
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

#[test]
fn config_systems_test() {
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

    config_systems! {
        (plugin = MyPlugin, schedule = Update)

        // Non-#[system] functions are unaffected
        fn get_world_name() -> &'static str {
            "World"
        }

        #[system(schedule = Startup)]
        fn hello_world()
        {
            println!("Hello, {}!", get_world_name());
        }

        #[system]
        fn get_time(
            time: Res<Time>,
            mut marker: ResMut<Marker>,
        ) {
            println!("The time is {}", time.elapsed_secs());
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