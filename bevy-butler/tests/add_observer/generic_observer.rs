use std::{any::type_name, fmt::Display};

use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_log::info;
use wasm_bindgen_test::wasm_bindgen_test;

include!("../common.rs");
use common::log_plugin;

#[derive(Resource)]
struct GenericResource<T>(pub bool, std::marker::PhantomData<T>);

#[derive(Event)]
struct GenericEvent<T>(pub T);

#[butler_plugin]
struct MyPlugin;

// Duplicated generics to test an issue that existed with deluxe
#[add_observer(generics = <&str, &str>, plugin = MyPlugin)]
#[add_observer(generics = <u8, u8>, plugin = MyPlugin)]
#[add_observer(generics = <bool, bool>, plugin = MyPlugin)]
fn test_observer<T: 'static + Sync + Send + Display, R>(person: On<GenericEvent<T>>, mut commands: Commands) {
    info!("{} = {}!", type_name::<R>(), person.0);
    info!("{} is also here", type_name::<R>());
    commands.insert_resource(GenericResource(true, std::marker::PhantomData::<T>));
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(
            Startup,
            |mut commands: Commands| {
                commands.trigger(GenericEvent("Hello"));
                commands.trigger(GenericEvent(52u8));
                commands.trigger(GenericEvent(true));
            },
        )
        .add_systems(
            PostStartup,
            (
                |res: Res<GenericResource<&'static str>>| assert!(res.0),
                |res: Res<GenericResource<u8>>| assert!(res.0),
                |res: Res<GenericResource<bool>>| assert!(res.0),
            ),
        )
        .run();
}
