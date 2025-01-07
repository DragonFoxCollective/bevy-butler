//! # bevy-butler
//! 
//! A crate for making Bevy systems more self-documenting.
//! 
//! ```
//! # use bevy_butler_proc_macro::*;
//! # use bevy_butler_core::*;
//! use bevy::prelude::*;
//! use bevy_butler::*;
//! 
//! #[system(schedule = Startup)]
//! fn hello_world()
//! {
//!     info!("Hello, world!");
//! }
//! 
//! #[derive(Resource)]
//! pub struct Hello(pub String);
//! 
//! pub struct MyPlugin;
//! 
//! #[butler_plugin]
//! impl Plugin for MyPlugin {
//!     fn build(&self, app: &mut App) {
//!         app.insert_resource(Hello("MyPlugin".to_string()));
//!     }
//! }
//! 
//! #[system(schedule = Update, plugin = MyPlugin)]
//! fn hello_plugin(name: Res<Hello>)
//! {
//!     info!("Hello, {}!", name.0);
//! }
//! 
//! #[system(schedule = Update, plugin = MyPlugin, transforms = after(hello_plugin))]
//! fn goodbye_plugin(name: Res<Hello>)
//! {
//!     info!("Goodbye, {}!", name.0);
//! }
//! 
//! fn main() {
//!     App::new()
//!         .add_plugins((BevyButlerPlugin, MyPlugin))
//!         .run();
//! }
//! ```

#[doc(hidden)]
pub use bevy_butler_core::__internal;

/// Include a system in a given [`Schedule`](bevy::prelude::Schedule). Optionally, define an
/// [`#[butler_plugin]`][butler_plugin] to be registered with.
/// 
/// # Attributes
/// ## `schedule` (Required)
/// Defines the [`Schedule`](bevy::prelude::Schedule) that the system should run in.
/// 
/// ## `plugin`
/// Defines a struct marked with [`#[butler_plugin]`](butler_plugin) that the
/// system should be registered with. If not defined, the system will be registered
/// with [`BevyButlerPlugin`].
/// 
/// ## `transforms`
/// Use to add additional definition methods to the system, such as [`run_if`](bevy::prelude::IntoSystemConfigs::run_if),
/// [`before`](bevy::prelude::IntoSystemConfigs::before) and [`after`](bevy::prelude::IntoSystemConfigs::after).
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
/// #[system(schedule = Startup, transforms = after(hello_world))]
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

/// Implements [`Plugin`](bevy::prelude::Plugin) on the struct to register any [`#[system]`](system) functons assigned to it.
/// 
/// If you want to perform additional setup in the [`build`](bevy::prelude::Plugin::build) function, you
/// can define an additional configuration function with [`configure_plugin`].
/// 
/// ```
/// # use bevy_butler_proc_macro::*;
/// #[butler_plugin]
/// pub struct MyPlugin;
/// ```
pub use bevy_butler_proc_macro::butler_plugin;

/// Adds a configuration function to run within an [`#[butler_plugin]`](butler_plugin)'s [`build`](bevy::prelude::Plugin::build) function.
/// 
/// ```
/// # use bevy_butler_proc_macro::*;
/// # use bevy::prelude::*;
/// #
/// #[derive(Resource)]
/// pub struct Hello(String);
/// 
/// #[butler_plugin]
/// pub struct MyPlugin;
/// 
/// #[configure_plugin(MyPlugin)]
/// fn configure(plugin: &MyPlugin, app: &mut App)
/// {
///     app.insert_resource(Hello("World".to_string()));
/// }
/// ```
pub use bevy_butler_proc_macro::configure_plugin;

/// [`Plugin`](bevy::prelude::Plugin) that enables the usage of [`#[system]`](system)
/// and [`#[butler_plugin]`](butler_plugin). It should be added to the [`App`](bevy::app::App) before any
/// `#[butler_plugin]` plugins are.
pub use bevy_butler_core::BevyButlerPlugin;