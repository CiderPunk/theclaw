use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets, game_manager::{Game, PlayState}, health::Health, scheduling::GameSchedule, ship::PlayerShip
};

pub struct GameUiPlugin;

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct LivesDisplay;

impl Plugin for GameUiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_game_ui)

      .add_systems(Update, health_update.in_set(GameSchedule::EntityUpdates))
      .add_systems(OnEnter(PlayState::Alive), lives_update);

      
  }
}

fn health_update(
  mut health_display: Single<&mut Text, With<HealthDisplay>>,
  player_health_query: Query<&Health, With<PlayerShip>>,
) {
  let Ok(health) = player_health_query.get_single() else {
    return;
  };
  health_display.0 = format!("Health: {}", health.0);
}

fn lives_update(
  mut life_display: Single<&mut Text, With<LivesDisplay>>,
  game: Single<&Game>,
) {

  life_display.0 = format!("Ships: {}", game.lives);
}



fn init_game_ui(mut commands: Commands, scene_assets: Res<SceneAssets>) {
  commands.spawn((
    HealthDisplay,
    Text::new("Health"),
    TextFont {
      font: scene_assets.font.clone(),
      font_size: 20.,
      ..default()
    },
    Node{
      position_type: PositionType::Absolute,
      bottom: Val::Px(12.0),
      left: Val::Px(12.0),
      ..default()
    },
  ));

  commands.spawn((
    LivesDisplay,
    Text::new("Lives"),
    TextFont {
      font: scene_assets.font.clone(),
      font_size: 20.,
      ..default()
    },
    Node{
      position_type: PositionType::Absolute,
      bottom: Val::Px(12.0),
      right: Val::Px(12.0),
      ..default()
    },
  ));

}
