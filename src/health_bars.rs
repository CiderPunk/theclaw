use bevy::{color::palettes::css::*, prelude::*};

use crate::{asset_loader::SceneAssets, health::Health, scheduling::GameSchedule, ship::PlayerShip, state::GameStateEvent};


const HEALTH_BAR_WIDTH_PER_HEALTH: f32 = 15. / 100.;


pub struct HealthBarsPlugin;

impl Plugin for HealthBarsPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_healthbars)
    .add_systems(Update, health_update.in_set(GameSchedule::EntityUpdates));
  }
}





#[derive(Component, Default)]
struct HealthBorder(f32);

#[derive(Component, Default)]
struct HealthBar(f32);


#[derive(Component, Default)]
struct CaptiveHealthBorder(f32);

#[derive(Component, Default)]
struct CaptiveHealthBar(f32);


fn health_update(
  //mut health_display: Single<&mut Text, With<HealthDisplay>>,
  player_health_query: Query<&Health, With<PlayerShip>>,
  healthbar_all: Single<(&mut HealthBar, &mut Node, &mut Visibility), Without<HealthBorder>>,
  healthbar_container_all: Single<(&mut HealthBorder, &mut Node), Without<HealthBar>>,
) {
  let Ok(health) = player_health_query.get_single() else {
    return;
  };
  let (mut healthbar, mut hb_node, mut visibility) = healthbar_all.into_inner();
  let (mut healthbar_container, mut hbc_node) = healthbar_container_all.into_inner();

  let mut force_health_update = false;
  if healthbar_container.0 != health.max {
    hbc_node.width = Val::Vw(HEALTH_BAR_WIDTH_PER_HEALTH * health.max);
    healthbar_container.0 = health.max;
    force_health_update = true;
  }
  if force_health_update || healthbar.0 != health.value {
    if health.value <= 0.{
      *visibility = Visibility::Hidden;
    }
    else{
      *visibility = Visibility::Visible;
    }
    hb_node.width = Val::Percent((health.value / health.max) * 100.);
    healthbar.0 = health.value;
  }
}



fn init_healthbars(
  mut commands: Commands,
  scene_assets: Res<SceneAssets>,
) {

  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      display: Display::Grid,
      grid_template_columns: vec![
        GridTrack::min_content(), 
        GridTrack::flex(1.0)
      ],
      grid_template_rows: vec![
        GridTrack::auto(),
        GridTrack::auto(),
      ],
      top: Val::Px(12.0),
      left: Val::Px(12.0),
      width: Val::Vw(30.0),
      height: Val::Px(80.),

      //border: UiRect::all(Val::Px(2.)),
      ..default()
    },
    //BorderRadius::all(Val::Px(5.)),
    //BorderColor(WHITE.into()),
  ))
  .with_children(|parent|{

    //first text
    parent.spawn((
      Node{
        display: Display::Grid,
        padding: UiRect::right(Val::Px(6.0)),
        ..default()
      },
      Text::new("Health"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: 20.,
        ..default()
      },
    ));


      //health meter
    parent.spawn((

      Node{
        display: Display::Grid,
        ..default()
      },

    ))
    .with_children(|parent|{
      parent.spawn((
        HealthBorder(0.),
        Node {
          width: Val::Vw(15.0),
          height: Val::Px(30.),
          border: UiRect::all(Val::Px(2.)),
          ..default()
        },
        BorderRadius::all(Val::Px(5.)),
        BorderColor(WHITE.into()),
      ))
      .with_children(|parent| {
        parent.spawn((
          HealthBar(0.),
          Node {
            margin: UiRect::all(Val::Px(3.)),
            border: UiRect::all(Val::Px(1.)),
            width: Val::Percent(100.0),
            height: Val::Px(20.0),
            ..default()
          },
          BorderRadius::all(Val::Px(5.)),
          BackgroundColor(Color::srgba(0., 0.7, 0., 0.2)),
          BorderColor(Color::srgba(0., 0.9, 0., 0.4)),
        ));
      });
    });
  
    parent.spawn((
      Node{
        display: Display::Grid,
        padding: UiRect::right(Val::Px(6.)),

        ..default()
      },
      Visibility::Hidden,
      Text::new("Captive"),
      TextFont {
        font: scene_assets.font.clone(),
        font_size: 20.,
        ..default()
      },
    ));
    //captive health meter
    parent.spawn((
      Node{
        display: Display::Grid,
        //margin: UiRect::all(Val::Px(3.)),
        ..default()
      },
    ))
    .with_children(|parent|{

      parent.spawn((
        CaptiveHealthBorder(0.),
        Node {
          width: Val::Vw(15.0),
          height: Val::Px(30.),
          border: UiRect::all(Val::Px(2.)),
          ..default()
        },
        BorderRadius::all(Val::Px(5.)),
        BorderColor(WHITE.into()),
        Visibility::Hidden,
      ))
      .with_children(|parent| {
        parent.spawn((
          CaptiveHealthBar(0.),
          Node {
            margin: UiRect::all(Val::Px(3.)),
            border: UiRect::all(Val::Px(1.)),
            width: Val::Percent(100.0),
            height: Val::Px(20.0),
            ..default()
          },
          BorderRadius::all(Val::Px(5.)),
          BackgroundColor(Color::srgba(0., 0.7, 0., 0.2)),
          BorderColor(Color::srgba(0., 0.9, 0., 0.4)),
          Visibility::Inherited,
        ));
      });
    });
  });
}
