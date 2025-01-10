#![cfg_attr(feature = "nightly", feature(used_with_arg))]

use bevy::prelude::{Res, ResMut, Resource};
use bevy_app::{App, Plugin, PostStartup, Startup};
use bevy_butler::{butler_plugin, system};

#[test]
fn butler_plugin_struct() {
    #[butler_plugin]
    struct MyPlugin;

    App::new().add_plugins(MyPlugin).run();
}

#[test]
fn butler_plugin_impl() {
    struct MyPlugin;

    #[derive(Resource)]
    struct Marker(pub bool);

    #[butler_plugin]
    impl Plugin for MyPlugin {
        fn build(&self, nonstandard_name: &mut App) {
            println!("INSERTING MARKER");
            nonstandard_name.insert_resource(Marker(false));
        }
    }

    #[system(plugin = MyPlugin, schedule = Startup)]
    fn sys(mut marker: ResMut<Marker>) {
        marker.0 = true;
    }

    let mut app = App::new();
    app.add_plugins(MyPlugin);
    app.add_systems(PostStartup, |marker: Res<Marker>| assert!(marker.0));
    app.run();
}
