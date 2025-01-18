use std::{any::type_name, fmt::Display};

use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::info;

use super::common::log_plugin;

#[derive(Resource)]
struct GenericResource<T>(pub T);

#[butler_plugin {
    build(
        insert_resource = GenericResource("Hello"),
        insert_resource = GenericResource(52u8),
        insert_resource = GenericResource(true),
    )
}]
struct MyPlugin;

#[system(generics = <&str>, plugin = MyPlugin, schedule = Startup, before = test_sys::<u8>)]
#[system(generics = <u8>, plugin = MyPlugin, schedule = Startup, after = test_sys::<&str>)]
#[system(generics = <bool>, plugin = MyPlugin, schedule = Startup, after(test_sys::<&str>), after = test_sys::<u8>)]
fn test_sys<T: 'static + Sync + Send + Display>(res: Res<GenericResource<T>>) {
    info!("{} = {}", type_name::<T>(), res.0);
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .run();
}