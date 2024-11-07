use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, movement::{AcceleratingObjectBundle, Acceleration, MaxSpeed, Velocity}};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);

#[derive(Component)]
pub struct Ship;
pub struct ShipPlugin;



impl Plugin for ShipPlugin{
  fn build(&self, app: &mut App){
    app
      .add_systems(Startup, spawn_ship)
      .add_systems(Update, movement_controls);
  }
}


fn movement_controls(mut query: Query<&mut Acceleration, With<Ship>>, keyboard_input:Res<ButtonInput<KeyCode>>, time: Res<Time>){
  let Ok(mut acc) = query.get_single_mut() else{
    return;
  };
  acc.value = Vec3::ZERO;
  if keyboard_input.pressed(KeyCode::KeyD){
    acc.value.x -= 400.0;
  }
  if keyboard_input.pressed(KeyCode::KeyA){
    acc.value.x += 400.0;
  }  

  if keyboard_input.pressed(KeyCode::KeyW){
    acc.value.z += 400.0;
  }
  if keyboard_input.pressed(KeyCode::KeyS){
    acc.value.z -= 400.0;
  }
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
    Ship,
  ));
}


