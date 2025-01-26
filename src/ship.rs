use std::f32::consts::PI;
use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, collision_detection::Player, movement::Acceleration, scheduling::GameSchedule};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);
const SHIP_ACCELERATION: f32 = 500.0;
const SHIP_DAMPING: f32 = 150.0;
const SHIP_MAX_SPEED: f32 = 40.0;
const SHIP_MAX_PITCH: f32 = 0.1 * PI;
const SHIP_PITCH_RATE: f32 = 1.6;




const BOUNDS_X_MIN:f32 = -20.;
const BOUNDS_X_MAX:f32 = 50.;
const BOUNDS_Z_MIN:f32 = -30.;
const BOUNDS_Z_MAX:f32 = 30.;
pub struct ShipPlugin;

impl Plugin for ShipPlugin{
  fn build(&self, app: &mut App){
    app
      .add_systems(Startup, spawn_ship)
      .add_systems(Update, (movement_controls, update_pitch).chain().in_set(GameSchedule::UserInput))
      .add_systems(Update, bounds_check.in_set(GameSchedule::BoundsCheck));
  }
}

fn spawn_ship(mut commands:Commands, scene_assets:Res<SceneAssets>){
  commands.spawn((
    Ship::default(),
    SceneRoot(scene_assets.ship.clone()),
    Transform::from_translation(STARTING_TRANSLATION),
    Acceleration{
      acceleration:Vec3::ZERO,
      damping: SHIP_DAMPING,
      max_speed: SHIP_MAX_SPEED,
    },
  ));
}

#[derive(Component, Default)]
#[require(Transform, Acceleration, Player)]
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
  transform.rotation = Quat::from_rotation_y(ship.pitch);
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


fn bounds_check(mut query:Query<&mut Transform, With<Ship>>){
  let Ok(mut transform) = query.get_single_mut()
  else{
    return;
  };

  let translation = transform.translation;
  if translation.x > BOUNDS_X_MAX{
    transform.translation.x = BOUNDS_X_MAX;
  }
  if translation.x < BOUNDS_X_MIN{
    transform.translation.x = BOUNDS_X_MIN;
  }
  if translation.z > BOUNDS_Z_MAX{
    transform.translation.z = BOUNDS_Z_MAX;
  }
  if translation.z < BOUNDS_Z_MIN{
    transform.translation.z = BOUNDS_Z_MIN;
  }


}



