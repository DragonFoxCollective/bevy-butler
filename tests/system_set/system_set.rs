use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::common::log_plugin;

#[derive(Resource, Default)]
struct Accumulator(pub u32);

#[butler_plugin(build = init_resource::<Accumulator>)]
struct MyPlugin;

system_set! {
    (plugin = MyPlugin, schedule = PreStartup, chain)

    #[system]
    fn system_one(mut acc: ResMut<Accumulator>) {
        acc.0 = 17;
        info!("(1) Accum: {} (expected {})", acc.0, 17);
        assert_eq!(acc.0, 17);
    }

    #[system]
    fn system_two(mut acc: ResMut<Accumulator>) {
        acc.0 *= 2; // 34
        info!("(2) Accum: {} (expected {})", acc.0, 34);
        assert_eq!(acc.0, 34);
    }

    #[system()]
    fn system_three(mut acc: ResMut<Accumulator>) {
        acc.0 -= 6; // 28
        info!("(3) Accum: {} (expected {})", acc.0, 28);
        assert_eq!(acc.0, 28);
    }

    #[system(run_if = || true)]
    fn system_four(mut acc: ResMut<Accumulator>) {
        acc.0 /= 4; // 7
        info!("(4) Accum: {} (expected {})", acc.0, 7);
        assert_eq!(acc.0, 7);
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}
