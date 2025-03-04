use bevy::prelude::*;

const GAME_START_LIVES:u32 = 99;
const GAME_RESPAWN_TIME:f32 = 4.;
use crate::{scheduling::GameSchedule, state::GameState};

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Playing), init_game)
      .add_systems(OnEnter(PlayState::Dead), start_respawn_timer)
      .add_systems(Update, (respawn_player).in_set(GameSchedule::EntityUpdates))
      .init_state::<PlayState>();
  }
}


#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum PlayState{
  #[default]
  NotInGame,
  Dead,
  Alive,
}


#[derive(Component)]
pub struct Game{
  pub score:u64,
  pub lives:u32,
  respawn_timer:Timer,
}


fn start_respawn_timer(mut game:Single<&mut Game>){
  game.respawn_timer.reset();
}

fn respawn_player(mut commands:Commands, mut game:Single<&mut Game>,  time:Res<Time>,  mut play_state: ResMut<NextState<PlayState>>){
  game.respawn_timer.tick(time.delta());
  if game.respawn_timer.just_finished(){
    if game.lives == 0{
      
      //TODO: goto end screen 
    }
    game.lives -= 1;
    play_state.set(PlayState::Alive);
  }
}

fn init_game(mut commands:Commands,   mut next_state: ResMut<NextState<PlayState>>,){
  commands.spawn(
    Game{ 
      score:0, 
      lives:GAME_START_LIVES, 
      respawn_timer:Timer::from_seconds(GAME_RESPAWN_TIME, TimerMode::Once),
    }
  );
  next_state.set(PlayState::Alive);
}