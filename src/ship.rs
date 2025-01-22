use std::f32::consts::PI;
use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, movement::Acceleration, ordering::GameInit};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);
const SHIP_ACCELERATION: f32 = 350.0;
const SHIP_DAMPING: f32 = 200.0;
const SHIP_MAX_SPEED: f32 = 40.0;


pub struct ShipPlugin;

impl Plugin for ShipPlugin{
  fn build(&self, app: &mut App){
    app
      .add_systems(Startup, spawn_ship.in_set(GameInit))
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
    translation,
    Acceleration{
      acceleration:Vec3::ZERO,
      damping: SHIP_DAMPING,
      max_speed: SHIP_MAX_SPEED,
    },
  ));
}

#[derive(Component, Default)]
#[require(Transform, Acceleration)]
struct Ship{
  pitch:f32
}

fn movement_controls(mut query:Query<&mut Acceleration, With<Ship>>, keyboard_input:Res<ButtonInput<KeyCode>>){
  let Ok(mut acceleration) = query.get_single_mut()
  else{
    return;
  };
  let mut acc = Vec3::ZERO;

  if keyboard_input.pressed(KeyCode::KeyD){
    acc.x -= 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyA){
    acc.x += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyW){
    acc.z += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyS){
    acc.z -= 1.;
  }
  acceleration.acceleration = acc.normalize_or_zero() * SHIP_ACCELERATION;
}


