use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, health::Health, scheduling::GameSchedule, ship::PlayerShip};

pub struct GameUiPlugin;

#[derive(Component)]
struct HealthDisplay;

impl Plugin for GameUiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_game_ui)
      .add_systems(Update, health_update.in_set(GameSchedule::EntityUpdates));
  }
}

fn health_update(
  mut score_display: Single<&mut Text, With<HealthDisplay>>,
  player_health_query: Query<&Health, With<PlayerShip>>,
) {
  let Ok(health) = player_health_query.get_single() else {
    return;
  };

  score_display.0 = format!("Health: {}", health.0);
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
  ));
}
