mod camera;
mod asset_loader;
mod ship;
mod movement;
mod scheduling;
mod collision_detection;
mod health;
mod bullet;
mod enemy;

use bevy::prelude::*;
use bullet::BulletPlugin;
use camera::CameraPlugin;
use asset_loader::AssetLoaderPlugin;
use collision_detection::CollsionDetectionPlugin;
use enemy::EnemyPlugin;
use scheduling::SchedulingPlugin;
use ship::ShipPlugin;
use movement::MovementPlugin;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1,0.0,0.15)))
    .insert_resource(AmbientLight{
      color: Color::default(),
      brightness:750.0,
    })
    .add_plugins((
      DefaultPlugins,
      SchedulingPlugin,
      CameraPlugin, 
      AssetLoaderPlugin, 
      MovementPlugin, 
      ShipPlugin,
      CollsionDetectionPlugin,
      BulletPlugin,
      EnemyPlugin,
    ))
    .run();
}
