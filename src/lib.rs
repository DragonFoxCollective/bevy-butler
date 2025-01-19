pub mod __internal;

use std::any::type_name;
use std::any::TypeId;

use __internal::BUTLER_REGISTRY;
use bevy_app::App;
use bevy_app::Plugin;

pub use bevy_butler_proc_macro::butler_plugin;

pub use bevy_butler_proc_macro::system;

pub trait ButlerPlugin: Plugin {
    fn register_butler_systems(app: &mut App, marker: TypeId) {
        let factories = BUTLER_REGISTRY.get_system_factories(marker);
        for system_factory in factories {
            system_factory(app);
        }
        bevy_log::debug!("{} ran {} factories", type_name::<Self>(), factories.len());
    }
}

#[cfg(all(target_arch="wasm32", not(feature="wasm-experimental")))]
compile_error!(
"WebAssembly support in bevy-butler is experimental and buggy.
If you wish to try it anyways, enable the `wasm-experimental` feature.
See also: https://github.com/TGRCdev/bevy-butler/issues/3
");

#[cfg(target_arch="wasm32")]
extern "C" {
    fn __wasm_call_ctors();
}

/// This is supposed to make the constructors work on WebAssembly
/// but all of the systems just disappear entirely in the Github
/// tests and it refuses to run on my PC
/// 
/// I tried man
#[cfg(target_arch="wasm32")]
#[doc(hidden)]
pub fn _initialize() {
    unsafe { __wasm_call_ctors(); }
}