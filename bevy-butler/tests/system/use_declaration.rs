use bevy_app::prelude::*;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

use super::common::log_plugin;

struct MyPlugin;

#[butler_plugin]
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Marker(8u8, false));
        app.insert_resource(Marker("Hello", false));
    }
}

#[derive(Resource)]
struct Marker<T>(T, bool);

mod function {
    use bevy_ecs::prelude::*;
    use bevy_log::prelude::*;
    use std::any::type_name;
    use std::fmt::Display;
    pub(super) fn test_sys<T: 'static + Sync + Send + Display>(mut res: ResMut<super::Marker<T>>) {
        info!("{} = {}", type_name::<T>(), res.0);
        res.1 = true;
    }
}

#[system(
    plugin = MyPlugin,
    schedule = Startup,
    generics = <u8>,
)]
#[system(
    plugin = MyPlugin,
    schedule = Startup,
    generics = <&str>,
)]
use function::test_sys;

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |marker: Res<Marker<u8>>| assert!(marker.1))
        .add_systems(PostStartup, |marker: Res<Marker<&'static str>>| {
            assert!(marker.1)
        })
        .run();
}
