# bevy-butler

A crate for making Bevy systems more self-documenting.

![Crates.io License](https://img.shields.io/crates/l/bevy-butler)
[![Crates.io Version](https://img.shields.io/crates/v/bevy-butler)](https://crates.io/crates/bevy-butler)
[![docs.rs](https://img.shields.io/docsrs/bevy-butler)](https://docs.rs/bevy-butler/latest/bevy_butler/)

```rust
use bevy::prelude::*;
use bevy_butler::*;

#[system(schedule = Startup)]
fn hello_world()
{
    info!("Hello, world!");
}

#[derive(Resource)]
pub struct Hello(pub String);

pub struct MyPlugin;

#[butler_plugin]
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Hello("MyPlugin".to_string()));
    }
}

#[system(schedule = Update, plugin = MyPlugin)]
fn hello_plugin(name: Res<Hello>)
{
    info!("Hello, {}!", name.0);
}

#[system(schedule = Update, plugin = MyPlugin, after = hello_plugin)]
fn goodbye_plugin(name: Res<Hello>)
{
    info!("Goodbye, {}!", name.0);
}

fn main() {
    App::new()
        .add_plugins((BevyButlerPlugin, MyPlugin))
        .run();
}
```