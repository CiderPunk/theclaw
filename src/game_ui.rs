use bevy::{color::palettes::css::{BLUE, RED}, prelude::*};

use crate::{
  asset_loader::SceneAssets, game_manager::{Game, PlayState, PointEvent}, health::Health, scheduling::GameSchedule, ship::PlayerShip
};

pub struct GameUiPlugin;

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct LivesDisplay;

#[derive(Component)]
struct ScoreDisplay;

impl Plugin for GameUiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_game_ui)
      .add_systems(Update, (health_update, score_update).in_set(GameSchedule::DespawnEntities))
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
  health_display.0 = format!("Health: {}", health.value);
}

fn score_update(mut score_display:Single<&mut Text, With<ScoreDisplay>>, game:Single<&Game> ){
  score_display.0 = format!("Score: {}", game.score);
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

  commands.spawn((

    Node{
      width: Val::Percent(100.),
      flex_direction: FlexDirection::Column,
      justify_content:JustifyContent::FlexStart,
      align_items:AlignItems::Center,
      ..default()

    },
    //Outline::new(Val::Px(1.), Val::ZERO, RED.into()),
  )).with_children(|parent| {
    parent.spawn((
      ScoreDisplay,
      Text::new("Score"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: 20.,
        ..default()
      },
      Node{ 
        margin: UiRect::all(Val::Px(5.)),
        ..default()
      },
      //Outline::new(Val::Px(1.), Val::ZERO, BLUE.into()),
    ));

  });


}
