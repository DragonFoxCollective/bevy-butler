#![cfg_attr(feature = "nightly", feature(used_with_arg))]

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test(unsupported = test)]
fn system_set_test() {
    use bevy::prelude::*;
    use bevy_butler::*;

    #[derive(Resource)]
    struct Marker(pub u8);

    struct MyPlugin;

    #[butler_plugin]
    impl Plugin for MyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(Marker(0));
        }
    }

    system_set! {
        (plugin = MyPlugin, schedule = Startup, chain)

        #[system]
        fn one(mut marker: ResMut<Marker>) {
            assert_eq!(marker.0, 0);
            marker.0 += 1;
        }

        #[system]
        fn two(mut marker: ResMut<Marker>) {
            assert_eq!(marker.0, 1);
            marker.0 += 1;
        }

        #[system]
        fn three(mut marker: ResMut<Marker>) {
            assert_eq!(marker.0, 2);
            marker.0 += 1;
        }
    }

    App::new()
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |marker: Res<Marker>| assert_eq!(marker.0, 3))
        .run();
}
