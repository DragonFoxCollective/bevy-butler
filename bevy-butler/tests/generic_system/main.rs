use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

include!("../common.rs");
use common::log_plugin;

#[derive(Resource)]
struct GenericResource<T>(pub T, pub bool);

struct MyPlugin;

#[butler_plugin]
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GenericResource("Hello", false));
        app.insert_resource(GenericResource(52u8, false));
        app.insert_resource(GenericResource(true, false));
    }
}

//#[add_system(generics = <&str, &str>, plugin = MyPlugin, schedule = Startup, before = test_sys::<u8, u8>)]
#[add_system(generics = <u8,u8>, plugin = MyPlugin, schedule = Startup, after = test_sys::<&str, &str>)]
fn test_sys<T: 'static + Sync + Send + Display, R>(mut res: ResMut<GenericResource<T>>) {
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
