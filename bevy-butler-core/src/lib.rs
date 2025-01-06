use std::marker::PhantomData;

use bevy::prelude::*;
use inventory::Collect;

pub mod __internal {
    pub use inventory;
}

pub struct ButlerSystem<T> {
    pub func: fn(&mut App) -> (),
    pub marker: PhantomData<T>,
}

pub struct GlobalButlerSystem {
    pub func: fn(&mut App) -> (),
}

inventory::collect!(GlobalButlerSystem);

impl<T: Sync + 'static> Collect for ButlerSystem<T>
{
    fn registry() -> &'static inventory::Registry {
        static REGISTRY: inventory::Registry = inventory::Registry::new();
        &REGISTRY
    }
}