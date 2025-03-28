use std::{any::type_name, fmt::Display};

use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::info;
use wasm_bindgen_test::wasm_bindgen_test;

use super::common::log_plugin;

#[derive(Resource)]
struct GenericResource<T>(pub T, pub bool);

#[butler_plugin {
    build(
        insert_resource = GenericResource("Hello", false),
        insert_resource(GenericResource(52u8, false)),
        insert_resource = GenericResource(true, false),
    )
}]
struct MyPlugin;

#[add_system(generics = <&str>, plugin = MyPlugin, schedule = Startup, before = test_sys::<u8>)]
#[add_system(generics = <u8>, plugin = MyPlugin, schedule = Startup, after = test_sys::<&str>)]
#[add_system(generics = <bool>, plugin = MyPlugin, schedule = Startup, after(test_sys::<&str>), after = test_sys::<u8>)]
fn test_sys<T: 'static + Sync + Send + Display>(mut res: ResMut<GenericResource<T>>) {
    info!("{} = {}", type_name::<T>(), res.0);
    res.1 = true;
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(
            PostStartup,
            (
                |res: Res<GenericResource<&'static str>>| assert!(res.1),
                |res: Res<GenericResource<u8>>| assert!(res.1),
                |res: Res<GenericResource<bool>>| assert!(res.1),
            ),
        )
        .run();
}
