mod camera;

use bevy::prelude::*;
use camera::CameraPlugin;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgb(0.1,0.0,0.15)))
    .insert_resource(AmbientLight{
      color: Color::default(),
      brightness:750.0,
    })
    
    .add_plugins((DefaultPlugins, CameraPlugin))
    .run();
}
