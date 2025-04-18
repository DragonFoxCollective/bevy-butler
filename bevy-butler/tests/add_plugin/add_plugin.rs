use bevy_butler::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Marker(String);

#[add_plugin(plugin = MyPlugin, init = HelloPlugin("world".to_string()))]
pub struct HelloPlugin(String);

#[butler_plugin]
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Marker(self.0.clone()));
    }
}

#[butler_plugin]
pub struct MyPlugin;

#[test]
fn test() {
    App::new()
        .add_plugins(MyPlugin)
        .add_systems(Startup, |marker: Res<Marker>| assert_eq!((*marker).0, "world"))
        .run();
}