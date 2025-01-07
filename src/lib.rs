#[doc(hidden)]
pub use bevy_butler_core::__internal;

/// Include a system in an [`#[auto_plugin]`](auto_plugin)'s [build](bevy::prelude::Plugin::build) function.
/// 
/// # Attributes
/// ## `schedule` (Required)
/// Defines the [`Schedule`](bevy::prelude::Schedule) that the system should run in.
/// 
/// ## `plugin`
/// Defines a [`Plugin`](bevy::prelude::Plugin) marked with [`auto_plugin`] that the
/// system should be registered with. If not defined, the system will be registered
/// with [`BevyButlerPlugin`].
/// 
/// ## `transforms`
/// Use to add additional definition methods to the system, such as [`run_if`](bevy::prelude::IntoSystemConfigs::run_if),
/// [`before`](bevy::prelude::IntoSystemConfigs::before) and [`after`](bevy::prelude::IntoSystemConfigs::after).
/// 
/// # Examples
/// 
/// ```
/// # use bevy_butler_proc_macro::*;
/// # use bevy_butler_core::*;
/// # use bevy::prelude::*;
/// #
/// #[auto_plugin]
/// pub struct MyPlugin;
/// 
/// #[derive(Resource)]
/// pub struct Hello(pub String);
/// 
/// #[system(schedule = Update, plugin = MyPlugin, transforms = run_if(|| true))]
/// fn hello_world(name: Res<Hello>)
/// {
///     info!("Hello, {}!", name.0);
/// }
/// 
/// #[system(schedule = Update, plugin = MyPlugin, transforms = run_if(|| true).after(hello_world))]
/// fn goodbye_world(name: Res<Hello>)
/// {
///     info!("Goodbye, {}!", name.0);
/// }
/// 
/// fn main() {
///     App::new()
///         .insert_resource(Hello("World".to_string()))
///         .add_plugins((BevyButlerPlugin, MyPlugin))
///         .run();
/// }
/// ```
/// 
/// This should print in the console:
/// ```text
/// Hello, World!
/// Goodbye, World!
/// ```
pub use bevy_butler_proc_macro::system;

/// Implements [`Plugin`](bevy::prelude::Plugin) on the struct to register any [`#[system]`](system) functons assigned to it.
/// 
/// If you want to perform additional setup in the [`build`](bevy::prelude::Plugin::build) function, you
/// can define an additional configuration function with [`configure_plugin`].
/// 
/// ```
/// # use bevy_butler_proc_macro::*;
/// #[auto_plugin]
/// pub struct MyPlugin;
/// ```
pub use bevy_butler_proc_macro::auto_plugin;

/// Adds a configuration function to run within an [`#[auto_plugin]`](auto_plugin)'s [`build`](bevy::prelude::Plugin::build) function.
/// 
/// ```
/// # use bevy_butler_proc_macro::*;
/// # use bevy::prelude::*;
/// #
/// #[derive(Resource)]
/// pub struct Hello(String);
/// 
/// #[auto_plugin]
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
/// and [`#[auto_plugin]`](auto_plugin). It should be added to the [`App`](bevy::app::App) before any
/// `#[auto_plugin]` plugins are.
pub use bevy_butler_core::BevyButlerPlugin;