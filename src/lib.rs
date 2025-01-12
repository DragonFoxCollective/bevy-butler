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

/// Register a system to a [`#[butler_plugin]`](butler_plugin)
/// to run with a given [`Schedule`](bevy_ecs::prelude::Schedule).
///
/// # Attributes
/// ## `schedule`
/// Defines the [`Schedule`](bevy_ecs::prelude::Schedule) that the system should run in.
///
/// ## `plugin`
/// Defines a struct marked with [`#[butler_plugin]`](butler_plugin) that the
/// system should be registered with.
///
/// ## Others
/// Any other attributes that don't match the above will be interpreted as system transforms.
/// For example, you can define ordering with `#[system(after = hello_world)]` or `#[system(after(hello_world))]`.
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
/// the block. Accepts all `#[system]` attributes, and will insert the given arguments onto
/// every contained `#[system]` attribute.
/// 
/// `plugin` and `schedule` can be overriden in the `#[system]` invocation, but transformations
/// will be applied after the transformations defined in `config_systems!`.
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
///     (plugin = MyPlugin, schedule = Startup)
///
///     #[system]
///     fn on_startup() {
///         info!("Hello, world!");
///     }
///
///     #[system(schedule = Update)]
///     fn on_update(time: Res<Time>) {
///         info!("The current time is {}", time.elapsed_secs());
///     }
/// }
/// ```
pub use bevy_butler_proc_macro::config_systems;

///<div class="warning">
/// 
/// This syntax is only available with the `nightly` feature. For the stable syntax, see [`config_systems!`](config_systems).
/// 
/// </div>
/// 
/// Provide default attributes for all [`#[system]`](system) invocations within
/// the block. Accepts all `#[system]` attributes, and will insert the given arguments onto
/// every contained `#[system]` attribute.
/// 
/// `plugin` and `schedule` can be overriden in the `#[system]` invocation, but transformations
/// will be applied after the transformations defined in `#[config_systems_block]`.
///
/// ```
/// #![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]
/// #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
/// # use bevy::prelude::*;
/// # use bevy_butler::*;
/// # use bevy_log::info;
///
/// #[butler_plugin]
/// struct MyPlugin;
///
/// #[config_systems_block(plugin = MyPlugin, schedule = Update)]
/// {
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
#[cfg(feature = "nightly")]
pub use bevy_butler_proc_macro::config_systems_block;

/// Groups all enclosed [`#[system]`](system) invocations into a system set, which
/// can have transformations applied to it.
/// 
/// Unlike [`config_systems!`](config_systems), instead of reconfiguring the contained
/// `#[system]` blocks, `system_set!` will wrap all the systems in one system set and
/// add it to the plugin under the given schedule. This can be used to run set-level
/// transformations, such as [`chain`][bevy_ecs::prelude::IntoSystemConfigs::chain]. However,
/// because of this, you cannot redefine `schedule` or `plugin`, as the entire set is
/// added under one invocation of `app.add_systems`.
/// 
/// Transforms can still be defined on both a system-level and the set-level.
/// 
/// ```
/// # use bevy::prelude::*;
/// # use bevy_butler::*;
/// #
/// # #[butler_plugin]
/// # struct MyPlugin;
/// // Adds (one, two, three).chain() to MyPlugin
/// // When run, these systems will print
/// // ```
/// // One
/// // Two
/// // Three
/// // ```
/// system_set! {
///     (plugin = MyPlugin, schedule = Startup, chain)
/// 
///     #[system]
///     fn one() {
///         info!("One");
///     }
/// 
///     #[system]
///     fn two() {
///         info!("Two");
///     }
/// 
///     #[system]
///     fn three() {
///         info!("Three");
///     }
/// }
/// ```
pub use bevy_butler_proc_macro::system_set;
