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

#[add_system(plugin = GamePlugin, schedule = Startup)]
fn enter_game(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::InGame);
}

#[wasm_bindgen_test(unsupported = test)]
fn test() {
    let mut app = App::new();

    app.add_plugins((StatesPlugin, GamePlugin));

    let world = app.world_mut();
    world.run_schedule(Startup);
    world.run_schedule(StateTransition);

    assert_eq!(
        *world.get_resource::<State<GameState>>().expect("GameState was not inserted"),
        GameState::InGame
    );
}
