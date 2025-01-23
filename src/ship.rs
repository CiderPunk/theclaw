use std::f32::consts::PI;
use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, movement::Acceleration, ordering::GameInit};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);
const SHIP_ACCELERATION: f32 = 500.0;
const SHIP_DAMPING: f32 = 150.0;
const SHIP_MAX_SPEED: f32 = 40.0;
const SHIP_MAX_PITCH: f32 = 0.25 * PI;
const SHIP_PITCH_RATE: f32 = 100.;


pub struct ShipPlugin;

impl Plugin for ShipPlugin{
  fn build(&self, app: &mut App){
    app
      .add_systems(Startup, spawn_ship.in_set(GameInit))
      .add_systems(Update, (movement_controls, update_pitch).chain());
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
  target_pitch:f32,
  pitch:f32,
}


fn update_pitch(mut query:Query<(&mut Ship, &mut Transform)>, time: Res<Time>){
  let Ok((mut ship, mut transform)) = query.get_single_mut()
  else{
    return;
  };
  let diff = ship.target_pitch - ship.pitch;
  let max_turn =  SHIP_PITCH_RATE * time.delta_secs();
  if max_turn > diff.abs(){
    ship.pitch = ship.target_pitch;
  }
  else{
    ship.pitch += diff.signum() * max_turn;
  }

  //transform.rotation = Quat::from_rotation_x(ship.pitch);

}

fn movement_controls(mut query:Query<(&mut Acceleration, &mut Ship)>, keyboard_input:Res<ButtonInput<KeyCode>>){
  let Ok((mut acceleration, mut ship)) = query.get_single_mut()
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
  acc = acc.normalize_or_zero();
  acceleration.acceleration = acc * SHIP_ACCELERATION;
  ship.target_pitch = acc.z * SHIP_MAX_PITCH;
}


