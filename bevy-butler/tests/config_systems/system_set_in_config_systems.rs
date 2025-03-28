use super::common::*;
use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[derive(Resource, Default)]
struct Counter(pub u8);

#[butler_plugin(build = init_resource::<Counter>)]
struct MyPlugin;

config_systems! {
    (plugin = MyPlugin, schedule = Startup)

    add_system_set! {
        (chain)

        #[add_system]
        fn system_one(mut counter: ResMut<Counter>) {
            info!("System one!");
            assert_eq!(counter.0, 0);
            counter.0 = 1;
        }

        #[add_system]
        fn system_two(mut counter: ResMut<Counter>) {
            info!("System two");
            assert_eq!(counter.0, 1);
            counter.0 = 2;
        }

        #[add_system]
        fn system_three(mut counter: ResMut<Counter>) {
            info!("System three");
            assert_eq!(counter.0, 2);
            counter.0 = 3;
        }
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |counter: Res<Counter>| {
            assert_eq!(counter.0, 3)
        })
        .run();
}
