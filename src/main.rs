mod camera;
mod asset_loader;
mod ship;
mod movement;

use bevy::prelude::*;
use camera::CameraPlugin;
use asset_loader::AssetLoaderPlugin;
use movement::MovementPlugin;
use ship::ShipPlugin;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1,0.0,0.15)))
    .insert_resource(AmbientLight{
      color: Color::default(),
      brightness:750.0,
    })
    .add_plugins((DefaultPlugins, CameraPlugin, AssetLoaderPlugin, ShipPlugin, MovementPlugin))
    .run();
}
