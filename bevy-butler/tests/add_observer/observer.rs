use std::time::Duration;

use bevy::{prelude::*, time::TimePlugin};
use bevy_app::ScheduleRunnerPlugin;
use bevy_butler::*;

use crate::common::log_plugin;

#[derive(Resource)]
struct Attendees(Vec<String>);

#[butler_plugin(
    build = insert_resource(Attendees(vec![
        "Harrier Du Bois".to_string(),
        "Kim Kitsuragi".to_string(),
        "Mack Torson".to_string(),
    ]))
)]
struct MyPlugin;

#[derive(Event)]
struct PersonEntered(String, usize);

#[add_observer(plugin = MyPlugin)]
fn greet_person(person: Trigger<PersonEntered>, mut exit: EventWriter<AppExit>) {
    info!("Hello, {}!", person.0);
    if person.1 == 0 {
        exit.write(AppExit::Success);
    }
}

#[add_system(
    plugin = MyPlugin,
    schedule = Update,
    run_if = |attendees: Res<Attendees>| !attendees.0.is_empty()
)]
fn attendees_arriving(mut commands: Commands, mut attendees: ResMut<Attendees>) {
    if let Some(attendee) = attendees.0.pop() {
        commands.trigger(PersonEntered(attendee, attendees.0.len()));
    }
}

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
            (|| -> () { panic!("Timed out") }).run_if(|time: Res<Time>| time.elapsed_secs() > 5.0f32),
        )
        .run();
}
