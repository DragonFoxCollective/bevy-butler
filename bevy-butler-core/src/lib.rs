#![feature(const_type_id)]
use std::any::TypeId;

use bevy::prelude::*;

pub use inventory;

#[derive(Debug)]
pub struct ButlerFunc(TypeId, * const ());

unsafe impl Sync for ButlerFunc {}
unsafe impl Send for ButlerFunc {}

impl ButlerFunc {
    pub const fn new<T: 'static>(func: fn(&T, &mut App) -> ()) -> Self {
        let func_ptr = unsafe { std::mem::transmute(func) };
        Self(TypeId::of::<T>(), func_ptr)
    }

    pub fn type_id(&self) -> TypeId {
        self.0
    }

    pub fn get_func<T: 'static>(&self) -> fn(&T, &mut App) -> () {
        assert_eq!(TypeId::of::<T>(), self.0);
        unsafe { std::mem::transmute(self.1) }
    }

    pub fn try_get_func<T: 'static>(&self) -> Option<fn(&T, &mut App) -> ()> {
        if self.0 == TypeId::of::<T>() {
            return Some(self.get_func());
        }
        None
    }
}

pub struct GlobalButlerSystem {
    pub func: fn(&mut App) -> (),
}

inventory::collect!(GlobalButlerSystem);
inventory::collect!(ButlerFunc);