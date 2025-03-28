# 0.6.0
(Unreleased)
### Breaking Changes

- Many macros have been changed to prevent conflicts with Bevy's derive helpers, and to better communicate their purpose.
    - `observer` -> `add_observer`
    - `resource` -> `add_resource`
    - `system` -> `add_system`
    - `event` -> `register_event`
    - `system_set` -> `add_system_set`

# 0.5.7
Released 2025-03-19
- Added `add_to_plugin` for adding a `Plugin` or `PluginGroup` to a butler plugin

# 0.5.6
Released 2025-02-08
- Lowered minimum Bevy version to `0.15.0`

# 0.5.5
Released 2025-02-03
- `#[system]` now has the `pipe_in` argument, which allows specifying a chain of systems to pipe input from. [[#14](https://github.com/TGRCdev/bevy-butler/issues/14)]
- Added `#[butler_plugin_group]` and `#[add_to_group]`, which allows defining PluginGroups using annotations, similar to `#[butler_plugin]` and `#[system]`.

# 0.5.4
Released 2025-01-31
- Enums can now be annotated with `#[butler_plugin]`, `#[event]`, `#[register_type]` and `#[resource]`. [[#18](https://github.com/TGRCdev/bevy-butler/issues/18)]

# 0.5.3
Released 2025-01-27

### Changes
- Added `#[register_type]`, which can be used to register `Reflect` types within a plugin's initialization [[#8](https://github.com/TGRCdev/bevy-butler/issues/8)]
- `#[resource]` and `#[event]` now support the `generics` argument [[#15](https://github.com/TGRCdev/bevy-butler/pull/15)]
- `#[observer]` can now be used on `use` statements.


# 0.5.2
Released 2025-01-26
### Additions
- `#[resource]` can be used to automatically initialize resources upon your plugin being added [[#7](https://github.com/TGRCdev/bevy-butler/issues/7)]
    ```rust
    #[derive(Resource)]
    #[resource(plugin = MyPlugin, init = Message("Hello, world!".to_string()))]
    struct Message(String);
    ```
- `#[event]` automatically registers events with `app.add_event::<MyEvent>()` [[#9](https://github.com/TGRCdev/bevy-butler/issues/9)]
    ```rust
    #[derive(Event)]
    #[event(plugin = MyPlugin)]
    struct UserJoined(String);
    ```
- `#[system]` now works with `use` statements [[#11](https://github.com/TGRCdev/bevy-butler/pull/11)]
    ```rust
    mod my_mod {
        pub(super) fn hello_world() {
            info!("Hello, world!");
        }
    }
    
    #[system(plugin = MyPlugin, schedule = Startup)]
    use my_mod::hello_world;
    ```

### Bug fixes
- `schedule` now accepts expressions instead of just type paths, for schedules like `OnEnter(MyState::MyVariant)` [[#12](https://github.com/TGRCdev/bevy-butler/issues/12)]

# 0.5.1
Released 2025-01-23
### Changes
- Fixed `#[butler_plugin]` stages not allowing methods with generic arguments like `register_type::<MyType>`. [[#4](https://github.com/TGRCdev/bevy-butler/issues/4), [#5](https://github.com/TGRCdev/bevy-butler/issues/5)]
- Added the `#[observer]` macro for registering [Observers](https://bevyengine.org/examples/ecs-entity-component-system/observers/). [[#6](https://github.com/TGRCdev/bevy-butler/issues/6)]

# 0.5.0
Released 2025-01-22

### Breaking Changes
- Experimental syntax like `#[config_systems_block]` has been removed.
- The `nightly` feature flag is no longer required, and has been removed.

### Changes
- `#[system]` can now register generic systems using `generics = <...>`
- `#[system]` can now be used multiple times on the same system, provided that the attribute arguments are distinct.
    - i.e. One system can be registered to multiple different schedules, plugins, generics, etc.
- `config_systems!` and `system_set!` now support nested invocations
- Experimental WASM support was added, which can be tested with the `wasm-experimental` feature flag.
- `inventory` is available as an alternative backend behind the `inventory` feature flag.

# 0.4.0
Released 2025-01-12

### Breaking Changes
- `#[config_systems]` has been renamed to `#[config_systems_block]`

### Changes
- `config_systems! {}` is a stable version of `#[config_systems_block]` that lets you define default attributes for a block of `#[system]`s.
  ```rust
  config_systems! {
      (schedule = Startup, plugin = MyPlugin)
      
      #[system]
      fn on_startup() {
          info!("Hello, world!");
      }
      
      #[system(schedule = Update)]
      fn on_update(time: Res<Time>) {
          info!("The current time is {}", time.elapsed_secs());
      }
  }
  ```
- `system_set! {}` lets you define a set of `#[system]`s and apply set-level transformations over them
  ```rust
  // This will be expanded into `(one, two, three).chain()`
  system_set! {
      (schedule = Startup, plugin = MyPlugin, chain)

      #[system]
      fn one() { info!("One!") }
      #[system]
      fn two() { info!("Two!") }
      #[system]
      fn three() { info!("Three!") }
  }
  ```
- Other attribute styles can now be included in `#[system]` and other system argument invocations. For example, `#[system(after = hello_world)]` can be written as `#[system(after(hello_world))]`
- Additional type checks have been implemented to prevent registering systems to non-ButlerPlugins, and to prevent registering systems to external ButlerPlugins.

# 0.3.0
Released 2025-01-09
### Breaking Changes
- `BevyButlerPlugin` is gone. Butler plugins can now be transparently treated as regular Bevy plugins.
- Global-scope systems are no longer supported. All `#[system]` invocations now require a `plugin` attribute defined.
    - This was done to improve `bevy-butler`'s usage within libraries by preventing libraries from polluting the global scope.

### Changes
- `bevy-butler` is now usable on stable! Nightly is still supported, but requires the `nightly` flag.
- `#[config_systems]` was added behind the `nightly` feature. It lets you define default attributes on a block that will apply to contained `#[system]` invocations.
    ```rust
    #[config_systems(plugin = MyPlugin)]
    {
        #[system(schedule = Startup)
        fn hello_world() {
            info!("Hello, world!");
        }

        #[system(schedule = Update)]
        fn print_time(time: Res<Time>) {
            info!("The current time is {}", time.elapsed_secs());
        }
    }
    ```
- `inventory` was replaced with `linkme`, which should widen the platforms that `bevy-butler` can work on.
    - Make sure to enable `nightly` feature if you're using a nightly compiler, else there will be linker errors.

# 0.2.0
Released 2025-01-08

### Changes
- `#[auto_plugin]` is now `#[butler_plugin]`
- `#[butler_plugin]` can now be applied to user-defined `impl Plugin` blocks
- The `transforms` argument has been removed from `#[system]`. Any extra name-value attributes are now assumed to be system transforms (i.e. `#[system(after = hello_world)]` becomes `.after(hello_world)`
- Rust edition was lowered to `2021`