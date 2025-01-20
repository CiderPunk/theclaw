use std::f32::consts::PI;
use bevy::{math::VectorSpace, prelude::*};

use crate::{asset_loader::SceneAssets, movement::Velocity};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);
const MAX_ACCELERATION: f32 = 8.0;
const MAX_ACCELERATION_COASTING: f32 = 1.0;
const TARGET_SPEED: f32 = 40.0;
const MAX_PITCH: f32 = 15.0;
const PITCH_SPEED: f32 = 5.0;


pub struct ShipPlugin;

impl Plugin for ShipPlugin{
  fn build(&self, app: &mut App){
    app
      .add_systems(Startup, spawn_ship)
      .add_systems(Update, movement_controls);
  }
}

fn spawn_ship(mut commands:Commands, scene_assets:Res<SceneAssets>){
  let mut translation = Transform::from_translation(STARTING_TRANSLATION);
  translation.rotate_x(PI * 0.5);
  translation.rotate_z(PI * 0.5);
  commands.spawn((
    Ship::default(),
    SceneRoot(scene_assets.ship.clone()),
    translation
  ));
}

#[derive(Component, Default)]
#[require(Transform, Velocity)]
struct Ship{
}

fn movement_controls(mut query:Query<&mut Velocity, With<Ship>>, keyboard_input:Res<ButtonInput<KeyCode>>){
  let Ok(mut velocity) = query.get_single_mut()
  else{
    return;
  };
  velocity.x = 0.;
  velocity.y = 0.;
  if keyboard_input.pressed(KeyCode::KeyD){
    velocity.x -= TARGET_SPEED;
  }
  if keyboard_input.pressed(KeyCode::KeyA){
    velocity.x += TARGET_SPEED;
  }
  if keyboard_input.pressed(KeyCode::KeyW){
    velocity.y += TARGET_SPEED;
  }
  if keyboard_input.pressed(KeyCode::KeyS){
    velocity.y -= TARGET_SPEED;
  }


}


