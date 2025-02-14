mod asset_loader;
mod bounds_check;
mod bullet;
mod camera;
mod collision_detection;
mod enemy;
mod game_ui;
mod health;
mod hook;
mod input;
mod movement;
mod scheduling;
mod ship;
mod sidewinder;
mod state;

use asset_loader::AssetLoaderPlugin;
use bevy::{asset::AssetMetaCheck, core::FrameCount, prelude::*};
use bounds_check::BoundsCheckPlugin;
use bullet::BulletPlugin;
use camera::CameraPlugin;
use collision_detection::CollsionDetectionPlugin;
use enemy::EnemyPlugin;
use game_ui::GameUiPlugin;
use health::HealthPlugin;
use hook::HookPlugin;

use input::GameInputPlugin;
use movement::MovementPlugin;
use scheduling::SchedulingPlugin;
use ship::ShipPlugin;
use sidewinder::SidewinderPlugin;
use state::{GameState, StatePlugin};


const APP_NAME:&str = "The Claw 2";
const CANVAS_ID:&str = "game_canvas";

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
    .insert_resource(AmbientLight {
      color: Color::default(),
      brightness: 750.0,
    })
    .add_plugins(
      DefaultPlugins
        .set( WindowPlugin{
          primary_window: Some(Window{
            title: APP_NAME.into(),
            name: Some(APP_NAME.into()),
            canvas: Some(CANVAS_ID.into()),
            fit_canvas_to_parent: true,
            visible: false,
            ..default()
          }),
          ..default()
        })
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }
      )
    )
    .add_plugins((
      StatePlugin,
      SchedulingPlugin,
      CameraPlugin,
      AssetLoaderPlugin,
      MovementPlugin,
      ShipPlugin,
      CollsionDetectionPlugin,
      BulletPlugin,
      EnemyPlugin,
      SidewinderPlugin,
      BoundsCheckPlugin,
      GameUiPlugin,
      HealthPlugin,
      HookPlugin,
    ))
    .add_plugins((GameInputPlugin,))
    .add_systems(Update, make_visible.run_if(in_state(GameState::Loading)))
    .run();
}


fn make_visible(mut window:Single<&mut Window>, frames:Res<FrameCount>){

  info!("frame {:?}", frames.0);
  if frames.0 == 1{
    window.visible = true;
  }

}
