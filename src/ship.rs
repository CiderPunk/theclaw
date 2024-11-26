use std::f32::consts::PI;
use bevy::{math::VectorSpace, prelude::*};

use crate::{asset_loader::SceneAssets, movement::{ MaxLinearAcceleration,  TargetVelocity, TargetVelocityObjectBundle, Velocity}};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);
const MAX_ACCELERATION: f32 = 5.0;
const TARGET_SPEED: f32 = 40.0;


#[derive(Component)]
pub struct Ship;
pub struct ShipPlugin;


#[derive(Component)]
pub struct Pitch{
  pub value:f32,
}

impl Pitch{
  pub fn new(value:f32)-> Self{
    Self{ value }
  }
}


impl Plugin for ShipPlugin{
  fn build(&self, app: &mut App){
    app
      .add_systems(Startup, spawn_ship)
      .add_systems(Update, movement_controls);
  }
}


fn movement_controls(mut query: Query<(&mut TargetVelocity, &mut Pitch), With<Ship>>, keyboard_input:Res<ButtonInput<KeyCode>>, time: Res<Time>){
  let Ok((mut target_velocity, mut pitch)) = query.get_single_mut() else{
    return;
  };


  let mut direction = Vec3::ZERO;

  target_velocity.value = Vec3::ZERO;
  if keyboard_input.pressed(KeyCode::KeyD){
    direction.x -= 1.0;
    //target_velocity.value.x -= TARGET_SPEED;
  }
  if keyboard_input.pressed(KeyCode::KeyA){
    direction.x += 1.0;
    //target_velocity.value.x += TARGET_SPEED;
  }  

  if keyboard_input.pressed(KeyCode::KeyW){
    direction.z += 1.0;
    //target_velocity.value.z += TARGET_SPEED;
  }
  if keyboard_input.pressed(KeyCode::KeyS){
    direction.z -= 1.0;
    //target_velocity.value.z -= TARGET_SPEED;
  }
  

}

fn spawn_ship(mut commands:Commands, scene_assets:Res<SceneAssets>){
  let mut translation = Transform::from_translation(STARTING_TRANSLATION);
  translation.rotate_x(PI *0.5);
  translation.rotate_z(PI *0.5);
  commands.spawn((
    TargetVelocityObjectBundle{
      velocity: Velocity::new(Vec3::ZERO),
      model:     SceneBundle{ 
        scene:scene_assets.ship.clone(),
        transform:translation,
        ..default()
      },
      target_volcity: TargetVelocity::new(Vec3::ZERO),
      max_accelleration: MaxLinearAcceleration::new(MAX_ACCELERATION),
    },
    Pitch::new(0.0),
    Ship,
  ));
}


