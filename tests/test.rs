use bevy::prelude::*;
use bevy_butler::{auto_plugin, configure_plugin, system};

#[test]
pub fn test() {
    #[derive(Resource)]
    pub struct Marker(pub bool);

    #[auto_plugin]
    #[derive(Debug)]
    pub struct TestPlugin;

    #[auto_plugin]
    #[derive(Debug)]
    pub struct OtherTestPlugin;

    #[configure_plugin(TestPlugin)]
    fn configure(plugin: &TestPlugin, app: &mut App) {
        app.insert_resource(Marker(false));
    }

    #[system(Startup, TestPlugin, run_if(|| true))]
    pub fn test_system(
        mut marker: ResMut<Marker>,
    ) {
        println!("HELLO, WORLD!!!!");
        marker.0 = true;
    }

    App::new()
        .add_plugins((TestPlugin, OtherTestPlugin))
        .add_systems(PostStartup, |marker: Res<Marker>| {
            println!("Testing marker");
            assert!(marker.0);
        })
        .run();
}