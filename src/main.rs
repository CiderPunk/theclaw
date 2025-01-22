mod camera;
mod asset_loader;
mod ship;
mod movement;
mod ordering;

use bevy::prelude::*;
use camera::CameraPlugin;
use asset_loader::AssetLoaderPlugin;
use ordering::{GameInit, GameLoading};
use ship::ShipPlugin;
use movement::MovementPlugin;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1,0.0,0.15)))
    .insert_resource(AmbientLight{
      color: Color::default(),
      brightness:750.0,
    })
    .configure_sets(Startup, GameInit.after(GameLoading) )
    .add_plugins((DefaultPlugins, CameraPlugin, AssetLoaderPlugin, MovementPlugin, ShipPlugin))
    .run();
}
