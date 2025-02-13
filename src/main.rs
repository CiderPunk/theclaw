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
use bevy::{asset::AssetMetaCheck, prelude::*};
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
use state::StatePlugin;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
    .insert_resource(AmbientLight {
      color: Color::default(),
      brightness: 750.0,
    })
    .add_plugins((
      DefaultPlugins.set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..Default::default()
      }),
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
    .run();
}
