#![doc = include_str!("../README.md")]
#![feature(const_type_id)]

mod core;
#[doc(hidden)]
pub use core::__internal;

/// Include a system in a given [`Schedule`](bevy_ecs::prelude::Schedule). Optionally, define an
/// [`#[butler_plugin]`][butler_plugin] to be registered with.
/// 
/// # Attributes
/// ## `schedule` (Required)
/// Defines the [`Schedule`](bevy_ecs::prelude::Schedule) that the system should run in.
/// 
/// ## `plugin`
/// Defines a struct marked with [`#[butler_plugin]`](butler_plugin) that the
/// system should be registered with. If not defined, the system will be registered
/// with [`BevyButlerPlugin`].
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
/// #[system(schedule = Startup)]
/// fn hello_world()
/// {
///     info!("Hello, world!");
/// }
/// 
/// #[system(schedule = Startup, after = hello_world)]
/// fn goodbye_world()
/// {
///     info!("Goodbye, world!");
/// }
/// 
/// #[system(schedule = Startup, plugin = MyPlugin)]
/// fn hello_plugin()
/// {
///     info!("Hello from MyPlugin!");
/// }
/// ```
pub use bevy_butler_proc_macro::system;

/// Macro for defining a Plugin that automatically registers [`#[system]`](system).
/// 
/// You can either mark a struct to generate a Plugin implementation, or
/// mark a Plugin implementation to include code for handling [`#[system]`](system) invocations.
/// 
/// ```
/// # use bevy_butler_proc_macro::*;
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

/// [`Plugin`](bevy_app::Plugin) that enables the usage of [`#[system]`](system)
/// and [`#[butler_plugin]`](butler_plugin). It should be added to the [`App`](bevy_app::App) before any
/// `#[butler_plugin]` plugins are.
pub use core::plugin::BevyButlerPlugin;