#![doc = include_str!("../README.md")]
#![doc(test(attr(cfg_attr(feature = "nightly", feature(used_with_arg)))))]
#![cfg_attr(feature = "nightly", feature(used_with_arg))]

#[doc(hidden)]
pub mod __internal;

/// Macro for defining a Plugin that automatically registers [`#[system]`](system).
///
/// You can either mark a struct to generate a Plugin implementation, or
/// mark a Plugin implementation to include code for handling [`#[system]`](system) invocations.
///
/// ```
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[derive(Resource)]
/// # struct Hello(pub String);
/// // Generates a plugin impl for a plugin struct
/// #[butler_plugin]
/// pub struct PluginOne;
///
/// pub struct PluginTwo;
///
/// // Inserts itself into a user-defined plugin impl
/// #[butler_plugin]
/// impl Plugin for PluginTwo {
///     fn build(&self, app: &mut App) {
///         app.insert_resource(Hello("World".to_string()));
///     }
/// }
/// ```
pub use bevy_butler_proc_macro::butler_plugin;

/// Include a system in a given [`Schedule`](bevy::prelude::Schedule). Optionally, define an
/// [`#[butler_plugin]`][butler_plugin] to be registered with.
///
/// # Attributes
/// ## `schedule` (Required)
/// Defines the [`Schedule`](bevy::prelude::Schedule) that the system should run in.
///
/// ## `plugin` (Required)
/// Defines a struct marked with [`#[butler_plugin]`](butler_plugin) that the
/// system should be registered with.
///
/// ## Extras
/// Any name-value attributes that don't match the above will be interpreted as system transforms.
/// For example, adding `after = hello_world` will resolve your system definiton as `system.after(hello_world)`.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_butler::*;
/// #
/// # #[butler_plugin]
/// # pub struct MyPlugin;
/// #
/// #[system(schedule = Startup, plugin = MyPlugin)]
/// fn hello_world()
/// {
///     info!("Hello, world!");
/// }
///
/// #[system(schedule = Startup, plugin = MyPlugin, after = hello_world)]
/// fn goodbye_world()
/// {
///     info!("Goodbye, world!");
/// }
/// ```
pub use bevy_butler_proc_macro::system;

/// Provide default attributes for all [`#[system]`](system) invocations within
/// the block. Supports all `#[system]` attributes.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_butler::*;
/// # use bevy_log::info;
///
/// #[butler_plugin]
/// struct MyPlugin;
///
/// config_systems! {
///     (plugin = MyPlugin, schedule = Update)
///
///     #[system(schedule = Startup)]
///     fn on_startup() {
///         info!("Hello, world!");
///     }
///
///     #[system]
///     fn on_update(time: Res<Time>) {
///         info!("The current time is {}", time.elapsed_secs());
///     }
/// }
/// ```
pub use bevy_butler_proc_macro::config_systems;

#[cfg(feature = "nightly")]
pub use bevy_butler_proc_macro::config_systems_block;

pub use bevy_butler_proc_macro::system_set;
