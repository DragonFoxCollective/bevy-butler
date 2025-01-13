#[cfg(all(feature="linkme", feature="inventory"))]
compile_error!("Features \"linkme\" and \"inventory\" are mutually exclusive. To use \"inventory\", disable default features.");
#[cfg(not(any(feature="linkme", feature="inventory")))]
compile_error!("bevy-butler needs a registry backend to construct the plugin registry. Please enable \"linkme\" or \"inventory\".");

use std::{collections::{HashMap, HashSet}, sync::LazyLock};
use bevy_app::App;
use std::any::TypeId;

#[cfg(feature="linkme")]
mod backend_linkme;
#[cfg(feature="linkme")]
pub use backend_linkme::*;
#[cfg(feature="linkme")]
pub use ::linkme;

#[cfg(feature="inventory")]
mod backend_inventory;
#[cfg(feature="inventory")]
pub use backend_inventory::*;
#[cfg(feature="inventory")]
pub use ::inventory;

pub type ButlerRegistry = HashMap<TypeId, HashSet<fn(&mut App)>>;

pub static BUTLER_REGISTRY: LazyLock<ButlerRegistry> = LazyLock::new(CONSTRUCT_BUTLER_REGISTRY);