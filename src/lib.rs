pub use bevy_butler_proc_macro::*;
use bevy::prelude::*;

pub use bevy_butler_core::*;
pub use bevy_butler_core::inventory;

pub struct BevyButlerPlugin;

impl Plugin for BevyButlerPlugin {
    fn build(&self, app: &mut App) {
        for system in inventory::iter::<GlobalButlerSystem> {
            (system.func)(app);
        }
    }
}