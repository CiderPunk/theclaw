use bevy::prelude::*;
pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<GameState>()
      .add_systems(PreStartup, init_game_state)
      .add_systems(Update, update_game_state)
      .add_event::<GameStateEvent>();
  }
}

#[derive(Event)]
pub struct GameStateEvent {
  state: GameState,
}

impl GameStateEvent {
  pub fn new(state: GameState) -> Self {
    Self { state }
  }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum GameState {
  #[default]
  Loading,
  Playing,
  // Paused,
}

fn init_game_state(mut next_state: ResMut<NextState<GameState>>) {
  info!("Setting initial game state");
  next_state.set(GameState::Loading);
}

fn update_game_state(
  mut ev_game_state: EventReader<GameStateEvent>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for &GameStateEvent { state } in ev_game_state.read() {
    info!("Switching game state {:?}", state);
    next_state.set(state);
  }
}
