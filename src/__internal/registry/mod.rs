use std::{collections::{HashMap, HashSet}, sync::LazyLock};
use bevy_app::App;
use std::any::TypeId;

#[cfg(not(feature="inventory"))]
mod backend_linkme;
#[cfg(not(feature="inventory"))]
pub use backend_linkme::*;
#[cfg(not(feature="inventory"))]
pub use ::linkme;

#[cfg(feature="inventory")]
mod backend_inventory;
#[cfg(feature="inventory")]
use backend_inventory::*;
#[cfg(feature="inventory")]
pub use ::inventory;

pub type ButlerRegistry = HashMap<TypeId, HashSet<fn(&mut App)>>;

pub static BUTLER_REGISTRY: LazyLock<ButlerRegistry> = LazyLock::new(CONSTRUCT_BUTLER_REGISTRY);