use bevy::prelude::*;
use bevy_butler::*;

#[butler_plugin_group]
pub struct GamePlugins;

#[add_plugin(to_group = GamePlugins, generics = <bool>, init = GenericPlugin(true))]
#[add_plugin(to_group = GamePlugins, generics = <u8>, init = GenericPlugin(5))]
#[add_plugin(to_group = GamePlugins, generics = <&'static str>, init = GenericPlugin("Hello, world!"))]
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
        .add_plugins(GamePlugins)
        .add_systems(Startup, |boolres: Res<GenericMarker<bool>>| {
            assert_eq!(boolres.0, true)
        })
        .add_systems(Startup, |u8res: Res<GenericMarker<u8>>| {
            assert_eq!(u8res.0, 5)
        })
        .add_systems(Startup, |strres: Res<GenericMarker<&'static str>>| {
            assert_eq!(strres.0, "Hello, world!")
        })
        .run();
}
