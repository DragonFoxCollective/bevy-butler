#![doc = include_str!("../README.md")]
#![doc(test(attr(cfg_attr(feature = "nightly", feature(used_with_arg)))))]
#![cfg_attr(feature = "nightly", feature(used_with_arg))]

#[doc(hidden)]
pub mod __internal;

pub use bevy_butler_proc_macro::*;