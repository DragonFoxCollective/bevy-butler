use bevy::prelude::*;
use bevy_butler::*;

#[derive(Resource)]
pub struct Marker(String);

#[add_plugin(to_plugin = MyPlugin)]
pub struct HelloPlugin(String);

impl Default for HelloPlugin {
    fn default() -> Self {
        Self("world".to_string())
    }
}

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
        .add_systems(Startup, |marker: Res<Marker>| {
            assert_eq!((*marker).0, "world")
        })
        .run();
}
