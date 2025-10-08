use bevy::prelude::*;
use bevy_butler::*;

#[butler_plugin]
pub struct GamePlugin;

#[add_plugin(to_plugin = GamePlugin, generics = <bool>, init = GenericPlugin(true))]
#[add_plugin(to_plugin = GamePlugin, generics = <u8>, init = GenericPlugin(5))]
#[add_plugin(to_plugin = GamePlugin, generics = <&'static str>, init = GenericPlugin("Hello, world!"))]
pub struct GenericPlugin<T>(T);

impl<T: 'static + Sync + Send + Clone> Plugin for GenericPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(GenericMarker(self.0.clone()));
    }
}

#[derive(Resource)]
pub struct GenericMarker<T>(T);

#[test]
fn test() {
    App::new()
        .add_plugins(GamePlugin)
        .add_systems(Startup, |boolres: Res<GenericMarker<bool>>| {
            assert!(boolres.0)
        })
        .add_systems(Startup, |u8res: Res<GenericMarker<u8>>| {
            assert_eq!(u8res.0, 5)
        })
        .add_systems(Startup, |strres: Res<GenericMarker<&'static str>>| {
            assert_eq!(strres.0, "Hello, world!")
        })
        .run();
}
