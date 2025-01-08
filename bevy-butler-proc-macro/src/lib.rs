use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, Item, ItemFn};

mod utils;

mod butler_plugin_impl;
mod system_impl;

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
#[proc_macro_attribute]
pub fn butler_plugin(args: TokenStream, item: TokenStream) -> TokenStream
{
    let parsed: Item = parse_macro_input!(item as Item);

    match parsed {
        Item::Impl(item_impl) => butler_plugin_impl::butler_plugin_impl(args, item_impl),
        Item::Struct(item_struct) => butler_plugin_impl::butler_plugin_struct(args, item_struct),
        
        _ => Error::new_spanned(
            parsed,
            "#[butler_plugin] can only be invoked on structs or `impl Plugin` blocks."
        )
            .to_compile_error()
            .into()
    }
}

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
/// with [`BevyButlerPlugin`](bevy_butler::BevyButlerPlugin).
/// 
/// ## Extras
/// Any name-value attributes that don't match the above will be interpreted as system transforms.
/// For example, adding `after = hello_world` will resolve your system definiton as `system.after(hello_world)`.
/// 
/// ```
/// # use bevy::prelude::*;
/// # use bevy_butler_proc_macro::*;
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
#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    system_impl::system_free_standing_impl(attr, parse_macro_input!(item as ItemFn))
}