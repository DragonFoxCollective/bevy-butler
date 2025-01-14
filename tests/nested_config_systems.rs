#![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]
#![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
#![cfg_attr(feature = "nightly", feature(used_with_arg))]

use bevy::prelude::*;
use bevy_butler::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test(unsupported = test)]
fn nested_config_systems() {
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
        (plugin = MyPlugin)

        config_systems! {
            (schedule = Startup)

            #[system]
            fn set_marker(mut marker: ResMut<Marker>) {
                marker.0 = true;
            }
        }
    }

    App::new()
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |marker: Res<Marker>| assert!(marker.0))
        .run();
}

#[cfg(feature = "nightly")]
#[wasm_bindgen_test(unsupported = test)]
fn nested_config_systems_block() {
    struct MyPlugin;

    #[butler_plugin]
    impl Plugin for MyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Marker(false));
        }
    }

    #[derive(Resource)]
    struct Marker(pub bool);

    #[config_systems_block(plugin = MyPlugin)]
    {
        #[config_systems_block(schedule = Startup)]
        {
            #[system]
            fn set_marker(mut marker: ResMut<Marker>) {
                marker.0 = true;
            }
        }
    }

    App::new()
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |marker: Res<Marker>| assert!(marker.0))
        .run();
}
