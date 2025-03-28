# bevy-butler

A set of procedural macros for making Bevy plugins and systems more self-documenting.

![Crates.io License](https://img.shields.io/crates/l/bevy-butler)
[![Crates.io Version](https://img.shields.io/crates/v/bevy-butler)](https://crates.io/crates/bevy-butler)
[![docs.rs](https://img.shields.io/docsrs/bevy-butler)](https://docs.rs/bevy-butler/latest/bevy_butler/)
![Crates.io MSRV](https://img.shields.io/crates/msrv/bevy-butler)


## Version Compatibility
| bevy | bevy-butler |
|------|-------------|
|`0.15`|   `0.5`     |

## Example
```rust
use bevy::prelude::*;
use bevy_butler::*;
use bevy_log::prelude::*;

#[butler_plugin]
pub struct MyPlugin;

#[derive(Resource)]
#[add_resource(plugin = MyPlugin, init = Hello("World".to_string()))]
pub struct Hello(pub String);

#[add_system(schedule = Update, plugin = MyPlugin)]
fn hello_plugin(name: Res<Hello>)
{
    info!("Hello, {}!", name.0);
}

#[add_system(schedule = Update, plugin = MyPlugin, after = hello_plugin)]
fn goodbye_plugin(name: Res<Hello>)
{
    info!("Goodbye, {}!", name.0);
}

fn main() {
    App::new()
        .add_plugins(MyPlugin)
        .run();
}
```

## WebAssembly support
WebAssembly support is currently locked behind the `wasm-experimental` flag. See the [relevant issue](https://github.com/TGRCdev/bevy-butler/issues/3#issuecomment-2601076962) for more information.