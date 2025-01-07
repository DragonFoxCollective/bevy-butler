use bevy::prelude::*;
use bevy_butler::{auto_plugin, configure_plugin, system, BevyButlerPlugin};

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

    #[system(schedule = Startup, plugin = TestPlugin, transforms = run_if(|| true))]
    fn test_system(
        mut marker: ResMut<Marker>,
    ) {
        println!("HELLO, WORLD!!!!");
        marker.0 = true;
    }

    #[system(schedule = Update)]
    fn assert_sys(marker: Res<Marker>, mut exit: EventWriter<AppExit>) {
        assert!(marker.0);
        exit.send(AppExit::Success);
    }

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(BevyButlerPlugin)
        .add_plugins((TestPlugin, OtherTestPlugin))
        .run();
}