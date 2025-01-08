#![doc = include_str!("../README.md")]
#![feature(const_type_id)]

mod core;
#[doc(hidden)]
pub use core::__internal;
pub use bevy_butler_proc_macro::*;
pub use core::plugin::BevyButlerPlugin;