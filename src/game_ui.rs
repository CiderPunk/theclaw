use bevy::{color::palettes::css::*, prelude::*};

use crate::{
  asset_loader::SceneAssets,
  game_manager::{Game, PlayState},
  health::Health,
  scheduling::GameSchedule,
  ship::PlayerShip,
};


const HEALTH_BAR_WIDTH_PER_HEALTH:f32 = 15. / 100.;

pub struct GameUiPlugin;


#[derive(Component, Default)]
struct HealthBorder(f32);

#[derive(Component, Default)]
struct HealthBar(f32);

#[derive(Component)]
struct LivesDisplay;

#[derive(Component)]
struct ScoreDisplay;

impl Plugin for GameUiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_game_ui)
      .add_systems(
        Update,
        (health_update, score_update).in_set(GameSchedule::DespawnEntities),
      )
      .add_systems(OnEnter(PlayState::Alive), lives_update);
  }
}

fn health_update(
  //mut health_display: Single<&mut Text, With<HealthDisplay>>,
  player_health_query: Query<&Health, With<PlayerShip>>,
  healthbar_all:Single<(&mut HealthBar, &mut Node), Without<HealthBorder>>,
  healthbar_container_all:Single<(&mut HealthBorder, &mut Node), Without<HealthBar>>,
) {
  let Ok(health) = player_health_query.get_single() else {
    return;
  };
  let (mut healthbar, mut hb_node) = healthbar_all.into_inner();
  let (mut healthbar_container, mut hbc_node) = healthbar_container_all.into_inner();

  let mut force_health_update = false;
  if healthbar_container.0 != health.max{
    hbc_node.width = Val::Vw(HEALTH_BAR_WIDTH_PER_HEALTH * health.max);
    healthbar_container.0 =  health.max;
    force_health_update = true;
  }

  if force_health_update || healthbar.0 != health.value{
    hb_node.width = Val::Percent((health.value / health.max) * 100.);
    healthbar.0 = health.value;
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
 // asset_server: Res<AssetServer>,
) {


  commands.spawn((
    HealthBorder(0.),
    Node{
      position_type: PositionType::Absolute,
      top: Val::Px(12.0),
      left: Val::Px(12.0),
      width:Val::Vw(15.0),
      height: Val::Px(30.0),
      border: UiRect::all(Val::Px(2.)),
      ..default()
    },
    
    BorderRadius::all(Val::Px(5.)),
    BorderColor(WHITE.into()),
  ))
  .with_children(|parent| {
    parent.spawn((
      HealthBar(0.),
      Node{
        margin: UiRect::all(Val::Px(3.)),
        border: UiRect::all(Val::Px(1.)),
        width:Val::Percent(100.0),
        height:Val::Px(20.0),
        ..default()
      },
      BorderRadius::all(Val::Px(5.)),
      BackgroundColor(Color::srgba(0.,0.7,0.,0.2)),
      BorderColor(Color::srgba(0.,0.9,0.,0.4)),
  
    ));
  });

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
