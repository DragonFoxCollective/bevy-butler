use bevy_butler::*;
use bevy::prelude::*;
use bevy_state::app::StatesPlugin;
use wasm_bindgen_test::wasm_bindgen_test;

#[butler_plugin]
struct GamePlugin;

#[insert_state(plugin = GamePlugin)]
#[derive(States, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum GameState {
    #[default]
    Loading,
    InGame
}

#[add_sub_state(plugin = GamePlugin)]
#[derive(SubStates, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[source(GameState = GameState::InGame)]
enum IsPaused {
    #[default]
    Running,
    Paused
}

#[add_system(plugin = GamePlugin, schedule = Startup)]
fn enter_game(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::InGame);
}

#[add_system(plugin = GamePlugin, schedule = Startup, after = enter_game)]
fn pause(
    mut next_state: ResMut<NextState<IsPaused>>
) {
    next_state.set(IsPaused::Paused);
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    let mut app = App::new();

    app.add_plugins((StatesPlugin, GamePlugin));

    let world = app.world_mut();
    world.run_schedule(Startup);
    world.run_schedule(StateTransition);

    assert_eq!(
        *world.get_resource::<State<IsPaused>>().expect("IsPaused was not inserted"),
        IsPaused::Paused
    );
}
