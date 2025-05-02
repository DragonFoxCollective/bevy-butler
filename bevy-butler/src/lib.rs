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
/// #[add_system(plugin = MyPlugin, schedule = Startup)]
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
/// #[add_system(plugin = MyPlugin, schedule = Startup)]
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
/// #[add_system(generics = <&'static str>, plugin = MyPlugin, schedule = Update)]
/// #[add_system(generics = <u32>, plugin = MyPlugin, schedule = Update)]
/// #[add_system(generics = <bool>, plugin = MyPlugin, schedule = Update)]
/// fn print_my_resource<T: 'static + Send + Sync + Display>(res: Res<GenericResource<T>>) {
///     info!("Resource: {}", res.0);
/// }
/// ```
///
/// ## `pipe_in`
/// One or more system pipes to use as inputs to the system.
///
/// Pipes are used in the order given, so `pipe_in(sys1, sys2)` would result in `sys1.pipe(sys2).pipe(<system>)`.
///
/// For more info, see the [Bevy Cheat Book](https://bevy-cheatbook.github.io/programming/system-piping.html) page on system piping.
///
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// fn get_name() -> String {
///     "World".to_string()
/// }
///
/// fn greet_name(name: In<String>) -> String {
///     format!("Hello, {}!", *name)
/// }
///
/// #[add_system(plugin = MyPlugin, schedule = Startup, pipe_in = [get_name, greet_name])]
/// fn print_greeting(greeting: In<String>) {
///     info!("{}", *greeting);
/// }
/// ```
///
/// ## System transforms
/// Any attribute that doesn't match the above is assumed to be a system transform function, like [`run_if`](bevy_ecs::prelude::IntoScheduleConfigs::run_if)
/// or [`after`](bevy_ecs::prelude::IntoScheduleConfigs::after).
/// ```rust
/// # use std::fmt::Display;
/// # use bevy_butler::*;
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[add_system(plugin = MyPlugin, schedule = Startup)]
/// fn system_one() {
///     info!("One!");
/// }
///
/// #[add_system(plugin = MyPlugin, schedule = Startup, after = system_one)]
/// fn system_two() {
///     info!("Two!");
/// }
///
/// #[add_system(plugin = MyPlugin, schedule = Startup, after(system_two))]
/// fn system_three() {
///     info!("Three!");
/// }
/// ```
///
pub use bevy_butler_proc_macro::add_system;

/// Registers an [observer](bevy_ecs::prelude::Observer) function to a [`#[butler_plugin]`](butler_plugin)-annotated [`Plugin`](bevy_app::prelude::Plugin).
///
/// # Usage
/// ## On a free-standing function
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # use bevy_log::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// # #[derive(Event)]
/// # struct Message {
/// #     content: String,
/// # }
/// #[add_observer(plugin = MyPlugin)]
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
/// #   use bevy_log::prelude::*;
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
/// #[add_observer(plugin = MyPlugin)]
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
pub use bevy_butler_proc_macro::add_observer;

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
/// #[insert_resource(plugin = MyPlugin)]
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
/// #[insert_resource(plugin = MyPlugin)]
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
/// #[insert_resource(plugin = MyPlugin)]
/// type MyResource = ExternalResource<usize>;
/// ```
///
/// # Arguments
/// ## `plugin` (Required)
/// A [`Plugin`](bevy_app::prelude::Plugin) annotated with [`#[butler_plugin]`](butler_plugin) to register this resource to.
///
/// ## `init`
/// By default, `#[insert_resource]` will use the [`Default`] value of the resource.
/// This can be overridden by specifying an `init` value.
///
/// ```rust
/// # use bevy_ecs::prelude::*;
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[derive(Resource)]
/// #[insert_resource(
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
/// #[insert_resource(plugin = MyPlugin, non_send)]
/// struct MyNonSendResource;
/// ```
pub use bevy_butler_proc_macro::insert_resource;

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
/// #[add_event(plugin = MyPlugin)]
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
/// #[add_event(plugin = MyPlugin)]
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
/// #[add_event(plugin = MyPlugin)]
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
pub use bevy_butler_proc_macro::add_event;

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

/// Implements `PluginGroup` and configures it to be used with [`add_plugin`]/[`add_plugin_group`].
///
/// # Usage
/// ## On a struct
/// Annotating a struct will automatically implement [`PluginGroup`](bevy_app::prelude::PluginGroup).
/// ```rust
/// # use bevy_butler::*;
/// #[butler_plugin_group]
/// struct MyPluginGroup;
/// ```
///
/// # Arguments
/// ## `name`
/// The internal name of the [`PluginGroup`](bevy_app::prelude::PluginGroup). Used to implement the [`name`](bevy_app::prelude::PluginGroup::name) function.
pub use bevy_butler_proc_macro::butler_plugin_group;

/// Adds the given `Plugin` to the target `Plugin`/`PluginGroup`
/// 
/// # Usage
/// ## On a struct
/// ```rust
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct BarPlugin;
/// # #[butler_plugin]
/// #[add_plugin(to_plugin = BarPlugin)]
/// struct FooPlugin;
/// ```
/// ## On an imported type
/// ```rust
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct BarPlugin;
/// # mod module {
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # #[derive(Default)]
/// # pub struct FooPlugin;
/// # }
/// #[add_plugin(to_plugin = BarPlugin)]
/// use module::FooPlugin;
/// ```
/// 
/// # Arguments
/// ## `to_plugin` / `to_group` (Required)
/// The target to register this Plugin to. Must be a [`butler_plugin`] if using `to_plugin`
/// and a [`butler_plugin_group`] if using `to_group`.
/// 
/// ## `generics`
/// A list of generic arguments to register the plugin with.
/// 
/// ## `init`
/// An expression to initialize the plugin with. If not set, will either default
/// to [`Default::default()`] or the plugin itself if the plugin is a zero-size struct.
pub use bevy_butler_proc_macro::add_plugin;

/// Adds the given `PluginGroup` to the target `Plugin`/`PluginGroup`
/// 
/// # Usage
/// ## On a struct
/// ```rust
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct BarPlugin;
/// # #[butler_plugin_group]
/// #[add_plugin_group(to_plugin = BarPlugin)]
/// struct FooPlugins;
/// ```
/// ## On an imported type
/// ```rust
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct BarPlugin;
/// # mod module {
/// # use bevy_butler::*;
/// # #[butler_plugin_group]
/// # #[derive(Default)]
/// # pub struct FooPlugins;
/// # }
/// #[add_plugin_group(to_plugin = BarPlugin)]
/// use module::FooPlugins;
/// ```
/// 
/// # Arguments
/// ## `to_plugin` / `to_group` (Required)
/// The target to register this Plugin to. Must be a [`butler_plugin`] if using `to_plugin`
/// and a [`butler_plugin_group`] if using `to_group`.
/// 
/// ## `generics`
/// A list of generic arguments to register the plugin with.
/// 
/// ## `init`
/// An expression to initialize the plugin with. If not set, will either default
/// to [`Default::default()`] or the plugin itself if the plugin is a zero-size struct.
pub use bevy_butler_proc_macro::add_plugin_group;

/// Adds the annotated state to a `#[butler_plugin]`
/// 
/// # Usage
/// ## On an enum
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct GamePlugin;
/// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
/// #[insert_state(plugin = GamePlugin)]
/// enum GameState {
///     #[default]
///     Loading,
///     InGame
/// }
/// ```
/// 
/// ## On a use statement
/// ```rust
/// # use bevy_butler::*;
/// # #[butler_plugin]
/// # struct GamePlugin;
/// # mod my_mod {
/// #   use bevy::prelude::*;
/// #   use bevy_butler::*;
/// #   #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
/// #   pub enum GameState {
/// #       #[default]
/// #       Loading,
/// #       InGame
/// #   }
/// # }
/// #[insert_state(plugin = GamePlugin)]
/// use my_mod::GameState;
/// ```
/// 
/// # Arguments
/// 
/// ## `plugin` (Required)
/// A [`Plugin`](bevy_app::prelude::Plugin) annotated with [`#[butler_plugin]`](butler_plugin) to register this state to.
/// 
/// ## `init`
/// By default, `#[insert_state]` will use [`init_state`](bevy_state::app::AppExtStates::init_state) to add the given state.
/// Setting the `init` argument will pass the given expression to [`insert_state`](bevy_state::app::AppExtStates::insert_state).
/// ```rust
/// # use bevy_butler::*;
/// # use bevy::prelude::*;
/// # #[butler_plugin]
/// # struct GamePlugin;
/// #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
/// #[insert_state(plugin = GamePlugin, init = MyState::Bar)]
/// enum MyState {
///     Foo,
///     Bar,
///     Baz
/// }
/// ```
/// 
/// ## `generics`
/// A list of generic arguments to register the state with. Used to register a generic state for multiple different types.
pub use bevy_butler_proc_macro::insert_state;

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
