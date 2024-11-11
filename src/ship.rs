use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};

use crate::{asset_loader::SceneAssets, movement::{AcceleratingObjectBundle, Acceleration, MaxSpeed, Velocity}};



const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);
const ACCELERATION: f32 = 500.0;
const DECCELERATION: f32 = 250.0;


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


fn movement_controls(mut query: Query<(&mut Acceleration, &mut Velocity, &mut Pitch), With<Ship>>, keyboard_input:Res<ButtonInput<KeyCode>>, time: Res<Time>){
  let Ok((mut acc, mut velocity, mut pitch)) = query.get_single_mut() else{
    return;
  };
  acc.value = Vec3::ZERO;
  if keyboard_input.pressed(KeyCode::KeyD){
    acc.value.x -= ACCELERATION;
  }
  if keyboard_input.pressed(KeyCode::KeyA){
    acc.value.x += ACCELERATION;
  }  

  if keyboard_input.pressed(KeyCode::KeyW){
    acc.value.z += ACCELERATION;
  }
  if keyboard_input.pressed(KeyCode::KeyS){
    acc.value.z -= ACCELERATION;
  }


  
  if acc.value == Vec3::ZERO{

    if velocity.value.length_squared() < 100.0{
      velocity.value = Vec3::ZERO;
    }
    let vel_norm = velocity.value.normalize_or_zero();
    acc.value = vel_norm * -DECCELERATION;
  }
  //max_speed.value = if  acc.value == Vec3::ZERO { 0.0 } else { 100.0};


}

fn spawn_ship(mut commands:Commands, scene_assets:Res<SceneAssets>){
  let mut translation = Transform::from_translation(STARTING_TRANSLATION);
  translation.rotate_x(PI *0.5);
  translation.rotate_z(PI *0.5);
  commands.spawn((
    AcceleratingObjectBundle{
      acceleration: Acceleration::new(Vec3::ZERO),
      velocity: Velocity::new(Vec3::ZERO),
      model:     SceneBundle{ 
        scene:scene_assets.ship.clone(),
        transform:translation,
        ..default()
      },
      max_speed: MaxSpeed::new(50.0),
    },
    Pitch::new(0.0),
    Ship,
  ));
}


