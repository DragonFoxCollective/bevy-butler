use std::time::Duration;

use bevy::{prelude::*, time::TimePlugin};
use bevy_app::ScheduleRunnerPlugin;
use bevy_butler::*;

use crate::common::log_plugin;

#[butler_plugin]
struct MyPlugin;

mod my_mod {
    use bevy::prelude::*;

    #[derive(Resource)]
    pub struct Attendees(pub Vec<String>);

    #[derive(Event)]
    pub struct PersonEntered(String, usize);

    pub fn greet_person(person: Trigger<PersonEntered>, mut exit: EventWriter<AppExit>) {
        info!("Hello, {}!", person.0);
        if person.1 == 0 {
            exit.send(AppExit::Success);
        }
    }

    pub fn attendees_arriving(mut commands: Commands, mut attendees: ResMut<Attendees>) {
        if let Some(attendee) = attendees.0.pop() {
            commands.trigger(PersonEntered(attendee, attendees.0.len()));
        }
    }
}

#[resource(plugin = MyPlugin, init = Attendees(vec![
    "Harrier Du Bois".to_string(),
    "Kim Kitsuragi".to_string(),
    "Mack Torson".to_string(),
]))]
use my_mod::Attendees;

#[system(
    plugin = MyPlugin,
    schedule = Update,
    run_if = |attendees: Res<Attendees>| !attendees.0.is_empty()
)]
use my_mod::attendees_arriving;

#[observer(plugin = MyPlugin)]
use my_mod::greet_person;

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins((
            TimePlugin,
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(0.1)),
        ))
        .add_plugins(MyPlugin)
        .add_systems(
            Update,
            (|| panic!("Timed out")).run_if(|time: Res<Time>| time.elapsed_secs() > 5.0f32),
        )
        .run();
}
