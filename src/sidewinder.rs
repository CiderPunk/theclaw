use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::{asset_loader::SceneAssets, enemy::*, movement::Velocity};

pub struct SidewinderPlugin;
const SIDEWINDER_SPANW_TIME_SECONDS:f32 = 2.;
const SIDEWINDER_SPIN_SPEED:f32 = 3.0;



#[derive(Deref, DerefMut)]
pub struct SpawnTimer(Timer);

impl Default for SpawnTimer{
  fn default()->Self{
    Self(Timer::from_seconds(SIDEWINDER_SPANW_TIME_SECONDS, TimerMode::Repeating))
  }
}

impl Plugin for SidewinderPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (spawn_sidewinder, spin_sidewinder));

  }
}

#[derive(Component)]
struct Sidewinder;


fn spin_sidewinder(mut query:Query<&mut Transform, With<Sidewinder>>, time:Res<Time>){
  for mut transform in query.iter_mut() {
    transform.rotate_local_x(SIDEWINDER_SPIN_SPEED * time.delta_secs());
  }
}

fn spawn_sidewinder(mut commands:Commands, 
  time:Res<Time>,
  mut timer: Local<SpawnTimer>, 
  scene_assets:Res<SceneAssets> 
){
  timer.tick(time.delta());
  if !timer.just_finished(){ return; }

  let mut rng = rand::thread_rng();
  let start_z =  rng.gen_range(ENEMY_START_POINT_Z_BOUNDS_MIN .. ENEMY_START_POINT_Z_BOUNDS_MAX);

  info!("Spawn sidewinder");
  commands.spawn((
    Sidewinder,
    SceneRoot(scene_assets.sidewinder.clone()),
    Transform::from_translation( Vec3::new(ENEMY_START_POINT_X,0., start_z))
      .with_rotation(Quat::from_rotation_z(PI)),
    Velocity( Vec3::new(20.0, 0.,0.)),
  ));
}
