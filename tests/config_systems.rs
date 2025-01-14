#![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]
#![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
#![cfg_attr(feature = "nightly", feature(used_with_arg))]

use bevy_butler::*;
use bevy_ecs::system::Resource;
use bevy_log::{Level, LogPlugin};
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(feature = "nightly")]
#[wasm_bindgen_test(unsupported = test)]
fn config_systems_block_attr() {
    use bevy::prelude::*;
    use bevy_app::{PostStartup, Startup};

    struct MyPlugin;

    #[butler_plugin]
    impl Plugin for MyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Marker(false));
        }
    }

    #[derive(Resource)]
    struct Marker(pub bool);

    #[config_systems_block(plugin = MyPlugin, schedule = Startup)]
    {
        #[system(schedule = Startup)]
        fn hello_world() {
            info!("Hello, world!");
        }

        #[system]
        fn goodbye_world(mut marker: ResMut<Marker>) {
            info!("Goodbye, world!");
            marker.0 = true;
        }
    }

    App::new()
        .add_plugins((LogPlugin {filter: "bevy_butler".to_string(), level: Level::TRACE, ..Default::default() }, MyPlugin))
        .add_systems(
            PostStartup,
            |marker: Res<Marker>| {
                assert!(marker.0, "Other systems failed to run");
            },
        )
        .run();
}

#[wasm_bindgen_test(unsupported = test)]
fn config_systems_function_macro() {
    use bevy::prelude::*;

    struct MyPlugin;

    #[butler_plugin]
    impl Plugin for MyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Marker(0));
        }
    }

    #[derive(Resource)]
    struct Marker(pub u8);

    config_systems! {
        (plugin = MyPlugin, schedule = Startup)

        // Non-#[system] functions are unaffected
        fn get_world_name() -> &'static str {
            "World"
        }

        #[system(schedule = Startup)]
        fn hello_world(mut marker: ResMut<Marker>)
        {
            marker.0 += 1;
        }

        #[system]
        fn get_time(
            mut marker: ResMut<Marker>,
        ) {
            marker.0 += 1;
        }
    }

    App::new()
        .add_plugins((LogPlugin {filter: "bevy_butler".to_string(), level: Level::TRACE, ..Default::default() }, MyPlugin))
        .add_systems(
            PostStartup,
            |marker: Res<Marker>| {
                assert_eq!(marker.0, 2);
            },
        )
        .run();
}
