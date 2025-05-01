//! Basically just testing that the most basic butler plugin still compiles

use bevy_butler::*;

#[allow(dead_code)]
#[butler_plugin]
struct MyPlugin;
