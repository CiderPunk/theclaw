use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, movement::Velocity};

pub struct BulletPlugin;


impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<ShootEvent>()
    .add_systems(Update, do_shooting);
  }
}

#[derive(Event)]
pub struct ShootEvent{
  pub start:Vec3,
  pub velocity:Vec3,
}

impl ShootEvent{
  pub fn new(start:Vec3, velocity:Vec3)->Self{
    Self{ start, velocity }
  }
}

#[derive(Component)]
pub struct Bullet;

fn do_shooting(mut commands:Commands, mut ev_shoot_events:EventReader<ShootEvent>, scene_assets:Res<SceneAssets> ){
  for &ShootEvent{ start, velocity } in ev_shoot_events.read(){
    info!("Spawing bullet");
    commands.spawn((
      Bullet,
      Mesh3d(scene_assets.bullet.clone()),
      MeshMaterial3d(scene_assets.bullet_material.clone()),
      Transform::from_translation(start),
      Velocity(velocity),
    ));
  }
}
