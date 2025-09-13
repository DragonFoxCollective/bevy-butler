# Migrating to [bevy_auto_plugin](https://github.com/StrikeForceZero/bevy_auto_plugin)

This project has been deprecated due to a lack of time and motivation to maintain it. However, bevy_auto_plugin recently implemented much of the functionality that bevy-butler has, so I plan to send users to that plugin and contribute there in the future.

Here is a quick list of which bevy-butler macros equate to which bevy_auto_plugin macros:
- `#[add_event]` -> [`#[auto_event]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_event.html)
- `#[add_observer]` -> [`#[auto_observer]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_observer.html)
- `#[add_system]` -> [`#[auto_system]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_system.html)
- `#[butler_plugin]` -> [`#[auto_plugin]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_plugin.html)
- `#[insert_resource]` -> [`#[auto_init_resource]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_init_resource.html)/[`#[auto_insert_resource]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_insert_resource.html)
- `#[insert_state]` -> [`#[auto_states]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_states.html)/[`#[auto_init_state]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_init_state.html)
- `#[register_type]` -> [`#[auto_register_type]`](https://docs.rs/bevy_auto_plugin/0.5.0/bevy_auto_plugin/modes/global/prelude/attr.auto_register_type.html)

Usage is fairly similar, but may need slight syntax changes.

# Unsupported macros

Some macros do not have direct replacements in bevy_auto_plugin.

- `#[add_plugin]`
- `#[butler_plugin_group]`