use bevy_butler::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Resource)]
struct Counter(pub u8);

#[butler_plugin(build = init_resource::<Counter>)]
struct MyPlugin;

#[test]
pub fn butler_plugin_test() {
    App::new()
        .add_plugins(MyPlugin)
        .add_systems(Startup, |counter: Res<Counter>| assert_eq!(counter.0, 0))
        .run();
}
