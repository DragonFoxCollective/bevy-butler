use bevy_butler::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;
use super::common::*;

#[butler_plugin(build = init_resource::<StepCounter>)]
struct MyPlugin;

#[derive(Resource, Default)]
struct StepCounter(pub u8);

system_set! {
    (plugin = MyPlugin, schedule = Startup, chain)

    #[system]
    fn system_one(mut counter: ResMut<StepCounter>) {
        info!("System one!");
        assert_eq!(counter.0, 0);
        counter.0 = 1;
    }

    system_set! {
        (chain)

        #[system]
        fn system_two(mut counter: ResMut<StepCounter>) {
            info!("System two!");
            assert_eq!(counter.0, 1);
            counter.0 = 2;
        }

        config_systems! {
            (run_if = || true)

            #[system]
            fn system_three(mut counter: ResMut<StepCounter>) {
                info!("System three!");
                assert_eq!(counter.0, 2);
                counter.0 = 3;
            }

            #[system]
            fn system_four(mut counter: ResMut<StepCounter>) {
                info!("System four!");
                assert_eq!(counter.0, 3);
                counter.0 = 4;
            }
        }
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}