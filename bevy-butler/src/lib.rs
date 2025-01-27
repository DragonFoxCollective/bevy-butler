#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod __internal;

/// Configures a plugin to be usable within bevy_butler's various macros
/// as a `plugin` argument.
///
/// # Usage
/// ## On a struct
/// Annotating a struct will automatically implement [`Plugin`](bevy_app::prelude::Plugin).
/// ```rust
/// # use bevy_butler::*;
/// #[butler_plugin]
/// struct MyPlugin;
/// ```
///
/// ## On an `impl Plugin` block
/// Annotating an `impl Plugin` block will transparently modify a user-defined [`Plugin`](bevy_app::prelude::Plugin) implementation
/// to support usage with butler macros.
/// ```rust
/// # use bevy_app::prelude::*;
/// # use bevy_butler::*;
/// struct MyPlugin;
///
/// #[butler_plugin]
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         /* ... */
///     }
/// }
/// ```
///
/// # Arguments
/// ## `build` | `finish` | `cleanup`
/// Butler plugins can define statements to run within their respective [`Plugin`](bevy_app::prelude::Plugin)
/// stages upon being added to an [`App`](bevy_app::prelude::App).
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[derive(Resource, Default)]
/// # struct Counter;
/// # #[derive(Resource)]
/// # struct Hello1(&'static str);
/// # #[derive(Resource)]
/// # struct Hello2(&'static str);
/// #[butler_plugin(
///     // Name-value style
///     build = init_resource::<Counter>,
///     // Becomes:
///     // app.init_resource::<Counter>();
///
///     // List style
///     finish(insert_resource(Hello1("World")), insert_resource(Hello2("World"))),
///     // Becomes:
///     // app.insert_resource(Hello1("World"));
///     // app.insert_resource(Hello2("World"));
/// )]
/// # struct MyPlugin;
/// ```
pub use bevy_butler_proc_macro::butler_plugin;

/// Registers a system to a [`#[butler_plugin]`](butler_plugin)-annotated [`Plugin`](bevy_app::prelude::Plugin).
///
/// # Usage
/// ## On a free-standing function
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #
/// #[system(plugin = MyPlugin, schedule = Startup)]
/// fn hello_world() {
///     info!("Hello, world!");
/// }
/// ```
/// 
/// ## On an imported system
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_ecs::prelude::*;
/// # use bevy_app::prelude::*;
/// # mod my_mod {
/// # pub(super) fn hello_world() {}
/// # }
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[system(plugin = MyPlugin, schedule = Startup)]
/// use my_mod::hello_world;
/// ```
/// # Arguments
/// ## `plugin` (Required)
/// A [`Plugin`](bevy_app::prelude::Plugin) annotated with [`#[butler_plugin]`](butler_plugin) to register this system to.
///
/// ## `schedule` (Required)
/// A [`Schedule`](bevy_ecs::prelude::Schedule) to run this system under.
///
/// ## `generics`
/// A list of generic arguments to register the system with. Used to register a generic system for multiple
/// different types.
/// ```rust
/// # use std::fmt::Display;
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[derive(Resource)]
/// struct GenericResource<T>(pub T);
///
/// #[system(generics = <&'static str>, plugin = MyPlugin, schedule = Update)]
/// #[system(generics = <u32>, plugin = MyPlugin, schedule = Update)]
/// #[system(generics = <bool>, plugin = MyPlugin, schedule = Update)]
/// fn print_my_resource<T: 'static + Send + Sync + Display>(res: Res<GenericResource<T>>) {
///     info!("Resource: {}", res.0);
/// }
/// ```
///
/// ## System transforms
/// Any attribute that doesn't match the above is assumed to be a system transform function, like [`run_if`](bevy_ecs::prelude::IntoSystemConfigs::run_if)
/// or [`after`](bevy_ecs::prelude::IntoSystemConfigs::after).
/// ```rust
/// # use std::fmt::Display;
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[system(plugin = MyPlugin, schedule = Startup)]
/// fn system_one() {
///     info!("One!");
/// }
///
/// #[system(plugin = MyPlugin, schedule = Startup, after = system_one)]
/// fn system_two() {
///     info!("Two!");
/// }
///
/// #[system(plugin = MyPlugin, schedule = Startup, after(system_two))]
/// fn system_three() {
///     info!("Three!");
/// }
/// ```
///
pub use bevy_butler_proc_macro::system;

/// Define a set of default [`#[system]`](system) arguments for the enclosed items
///
/// # Usage
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #
/// config_systems! {
///     (plugin = MyPlugin, schedule = Startup)
///
///     #[system]
///     fn system_foo() {
///         info!("Foo");
///     }
///
///     // Default arguments can be overridden
///     #[system(schedule = PostStartup)]
///     fn system_bar() {
///         info!("Bar");
///     }
/// }
/// ```
///
/// Note that `config_systems!` does not apply any sort of ordering or grouping of the enclosed systems.
/// If you want to apply set-level transformations like [`chain`](bevy_ecs::prelude::IntoSystemSetConfigs::chain),
/// see [`system_set!`](system_set).
///
/// # Arguments
/// `config_systems!` accepts any arguments that [`#[system]`](system) does. If any transforms are
/// provided, the `config_systems!` transforms will be applied **before** the individual `#[system]` attributes.
pub use bevy_butler_proc_macro::config_systems;

/// Wrap a set of [`#[system]`](system) functions into an anonymous system set, and apply set-level transformations.
///
/// # Usage
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_log::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// system_set! {
///     (plugin = MyPlugin, schedule = Update, chain)
///
///     #[system]
///     fn system_one() {
///         info!("One!");
///     }
///
///     #[system]
///     fn system_two() {
///         info!("Two!");
///     }
///
///     #[system(run_if = || true)]
///     fn system_three() {
///         info!("Three!");
///     }
/// }
///
/// // Equivalent set:
/// # let _ =
/// (system_one, system_two, system_three.run_if(|| true)).chain()
/// # ;
/// ```
///
/// Because this macro wraps all the enclosed systems in a single set,
/// the `plugin` and `schedule` arguments cannot be overridden.
///
/// `system_set!` also supports nested invocations of itself and [`config_systems!`](config_systems).
///
/// # Arguments
/// `system_set!` accepts arguments the same way that [`#[system]`](system) does. However,
/// any transforms defined will be applied to the overall set, NOT to the individual systems.
/// To apply the given arguments to every individual system, see [`config_systems!`](config_systems).
pub use bevy_butler_proc_macro::system_set;

/// Registers an [observer](bevy_ecs::prelude::Observer) function to a [`#[butler_plugin]`](butler_plugin)-annotated [`Plugin`](bevy_app::prelude::Plugin).
/// 
/// # Usage
/// ## On a free-standing function
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # #[derive(Event)]
/// # struct Message {
/// #     content: String,
/// # }
/// #[observer(plugin = MyPlugin)]
/// fn receive_message(message: Trigger<Message>) {
///     info!("Message received: {}", message.content);
/// }
/// ```
/// ## On an imported function
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # mod my_mod {
/// #   use bevy::prelude::*;
/// #
/// #   #[derive(Event)]
/// #   pub(super) struct Message {
/// #       content: String,
/// #   }
/// #
/// #   pub(super) fn receive_message(message: Trigger<Message>) {
/// #       info!("Message received: {}", message.content);
/// #   }
/// # }
/// #[observer(plugin = MyPlugin)]
/// use my_mod::receive_message;
/// ```
/// 
/// For more information about Observers, see the [Bevy example](https://bevyengine.org/examples/ecs-entity-component-system/observers/).
/// 
/// # Arguments
/// ## `plugin` (Required)
/// A [`Plugin`](bevy_app::prelude::Plugin) annotated with [`#[butler_plugin]`](butler_plugin) to register this observer to.
///
/// ## `generics`
/// A list of generic arguments to register the observer with. Used to register a generic observer for multiple
/// different types.
pub use bevy_butler_proc_macro::observer;

/// Registers the annotated [`Resource`](bevy_ecs::prelude::Resource) to a [`#[butler_plugin]`](butler_plugin) and
/// initializes it upon the plugin being added.
/// 
/// # Usage
/// ## On a struct
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[derive(Resource, Default)]
/// #[resource(plugin = MyPlugin)]
/// struct Counter(pub u8);
/// ```
/// 
/// ## On an imported type
/// ```rust
/// # use bevy_butler::*;
/// # mod my_mod {
/// #   use bevy_ecs::prelude::*;
/// #   
/// #   #[derive(Resource, Default)]
/// #   pub(super) struct ModResource;
/// # }
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[resource(plugin = MyPlugin)]
/// use my_mod::ModResource;
/// ```
/// 
/// ## On a type alias
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # #[derive(Resource, Default)]
/// # struct ExternalResource<T>(T);
/// #[resource(plugin = MyPlugin)]
/// type MyResource = ExternalResource<usize>;
/// ```
/// 
/// # Arguments
/// ## `plugin` (Required)
/// A [`Plugin`](bevy_app::prelude::Plugin) annotated with [`#[butler_plugin]`](butler_plugin) to register this resource to.
/// 
/// ## `init`
/// By default, `#[resource]` will use the [`Default`] value of the resource.
/// This can be overridden by specifying an `init` value.
/// 
/// ```rust
/// # use bevy_ecs::prelude::*;
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[derive(Resource)]
/// #[resource(
///     plugin = MyPlugin,
///     init = Message("Hello, world!".to_string())
/// )]
/// struct Message(String);
/// ```
/// 
/// ## `generics`
/// A list of generic arguments to register the resource with. Used to register a generic resource for multiple
/// different types.
/// 
/// ## `non_send`
/// If your resource should not be sent between threads, including `non_send` will register it using
/// [`init_non_send_resource`](bevy_app::prelude::App::init_non_send_resource)/
/// [`insert_non_send_resource`](bevy_app::prelude::App::insert_non_send_resource).
/// Can be written as `non_send`, `non_send = <bool>` or `non_send(<bool>)`.
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_ecs::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[derive(Resource, Default)]
/// #[resource(plugin = MyPlugin, non_send)]
/// struct MyNonSendResource;
/// ```
pub use bevy_butler_proc_macro::resource;

/// Registers the annotated [`Event`](bevy_ecs::prelude::Event) upon the
/// given [`#[butler_plugin]`](butler_plugin) being built.
/// 
/// # Usage
/// ## On a struct
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[derive(Event)]
/// #[event(plugin = MyPlugin)]
/// struct MessageReceived(String);
/// ```
/// 
/// ## On an imported type
/// ```rust
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # mod my_mod {
/// # use bevy_ecs::prelude::*;
/// # #[derive(Event)]
/// # pub struct ModMessageReceived(String);
/// # }
/// #[event(plugin = MyPlugin)]
/// use my_mod::ModMessageReceived;
/// ```
/// 
/// ## On a type alias
/// ```rust
/// # use bevy_butler::*;
/// # use bevy_ecs::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # #[derive(Event)]
/// # struct ExternalEventMessage<T>(T);
/// #[event(plugin = MyPlugin)]
/// type MyMessage = ExternalEventMessage<String>;
/// ```
/// 
/// # Arguments
/// ## `plugin` (Required)
/// A [`Plugin`](bevy_app::prelude::Plugin) annotated with [`#[butler_plugin]`](butler_plugin) to register this resource to.
/// 
/// ## `generics`
/// A list of generic arguments to register the event with. Used to register a generic event for multiple
/// different types.
pub use bevy_butler_proc_macro::event;

/// Registers the annotated `Reflect` type into the app's type registry for reflection.
/// 
/// # Usage
/// ## On a struct
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[derive(Reflect)]
/// #[register_type(plugin = MyPlugin)]
/// struct Name(String);
/// ```
/// ## On an imported type
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # mod my_mod {
/// # use bevy::prelude::*;
/// # #[derive(Reflect)]
/// # pub struct Name(String);
/// # }
/// #[register_type(plugin = MyPlugin)]
/// use my_mod::Name;
/// ```
/// ## On a type alias
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # #[derive(Reflect)]
/// # struct GenericContainer<T>(T);
/// #[register_type(plugin = MyPlugin)]
/// type MyName = GenericContainer<String>;
/// ```
/// 
/// # Arguments
/// ## `plugin` (Required)
/// A [`Plugin`](bevy_app::prelude::Plugin) annotated with [`#[butler_plugin]`](butler_plugin) to register this type to.
/// 
/// ## `generics`
/// A list of generic arguments to register the reflect type with. Used to register a generic reflect type for multiple
/// different types.
pub use bevy_butler_proc_macro::register_type;

#[cfg(all(target_arch = "wasm32", not(feature = "wasm-experimental")))]
compile_error!(
    "WebAssembly support in bevy-butler is experimental and buggy.
If you wish to try it anyways, enable the `wasm-experimental` feature.
See also: https://github.com/TGRCdev/bevy-butler/issues/3
"
);

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn __wasm_call_ctors();
}

/// This is supposed to make the constructors work on WebAssembly
/// but all of the systems just disappear entirely in the Github
/// tests and it refuses to run on my PC
///
/// I tried man
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}
