use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use super::common::*;

#[butler_plugin(build = init_resource::<Accumulator>)]
struct MyPlugin;

#[derive(Resource, Default)]
struct Accumulator(pub u32);

config_systems! {
    (plugin = MyPlugin, schedule = PreStartup)

    #[system]
    fn system_prestartup(mut acc: ResMut<Accumulator>) {
        info!("Pre-startup!");
        acc.0 = 17;
    }

    config_systems! {
        (schedule = Startup)

        #[system]
        fn system_startup_one(mut acc: ResMut<Accumulator>) {
            info!("Startup one!");
            acc.0 *= 2; // 34
        }

        config_systems! {
            #[system(after = system_startup_one)]
            fn system_startup_two(mut acc: ResMut<Accumulator>) {
                info!("Startup two!");
                acc.0 -= 6; // 28
            }

            config_systems! {
                (schedule = PostStartup)

                #[system]
                fn system_startup_three(mut acc: ResMut<Accumulator>) {
                    info!("Startup three!");
                    acc.0 /= 4; // 7
                }
            }
        }
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(
            PostStartup,
            (|acc: Res<Accumulator>| assert_eq!(acc.0, 7)).after(system_startup_three),
        )
        .run();
}
