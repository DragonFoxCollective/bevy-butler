#[macro_use]
pub use bevy_butler_proc_macro::*;
use bevy::prelude::*;

pub use bevy_butler_core::*;

pub struct BevyButlerPlugin;

impl Plugin for BevyButlerPlugin {
    fn build(&self, app: &mut App) {
        for system in inventory::iter::<GlobalButlerSystem> {
            (system.func)(app);
        }
    }
}

#[test]
pub fn test() {
    #[auto_plugin]
    pub struct TestPlugin;

    #[derive(Resource)]
    pub struct Marker(pub bool);

    #[system(Startup, TestPlugin)]
    pub fn test_system(
        mut marker: ResMut<Marker>,
    ) {
        println!("HELLO, WORLD!!!!");
        marker.0 = true;
    }

    App::new()
        .insert_resource(Marker(false))
        .add_plugins(TestPlugin)
        .add_systems(PostStartup, |marker: Res<Marker>| {
            println!("Testing marker");
            assert!(marker.0);
        })
        .run();
}