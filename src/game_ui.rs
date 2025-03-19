use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets,
  game_manager::{Game, PlayState},
  scheduling::GameSchedule,
};

pub struct GameUiPlugin;

#[derive(Component)]
struct LivesDisplay;

#[derive(Component)]
struct ScoreDisplay;

impl Plugin for GameUiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_game_ui)
      .add_systems(Update,score_update.in_set(GameSchedule::DespawnEntities))
      .add_systems(OnEnter(PlayState::Alive), lives_update);
  }
}

fn score_update(mut score_display: Single<&mut Text, With<ScoreDisplay>>, game: Single<&Game>) {
  score_display.0 = format!("Score: {}", game.score);
}

fn lives_update(mut life_display: Single<&mut Text, With<LivesDisplay>>, game: Single<&Game>) {
  life_display.0 = format!("Ships: {}", game.lives);
}

fn init_game_ui(
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
  _asset_server: Res<AssetServer>,
) {

  //let image:Handle<Image> = asset_server.load("ui/life.png");

  commands.spawn((
    LivesDisplay,
    Text::new("Lives"),
    TextFont {
      font: scene_assets.font.clone(),
      font_size: 20.,
      ..default()
    },
    Node {
      position_type: PositionType::Absolute,
      bottom: Val::Px(12.0),
      right: Val::Px(12.0),
      ..default()
    },
  ));

  commands
    .spawn((
      Node {
        width: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Center,
        ..default()
      },
      //Outline::new(Val::Px(1.), Val::ZERO, RED.into()),
    ))
    .with_children(|parent| {
      parent.spawn((
        ScoreDisplay,
        Text::new("Score"),
        TextFont {
          font: scene_assets.font.clone(),
          font_size: 20.,
          ..default()
        },
        Node {
          margin: UiRect::all(Val::Px(5.)),
          ..default()
        },
        //Outline::new(Val::Px(1.), Val::ZERO, BLUE.into()),
      ));
    });

}
