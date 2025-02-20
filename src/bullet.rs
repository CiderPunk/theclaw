use bevy::prelude::*;

use crate::{
  asset_loader::SceneAssets, bounds_check::BoundsDespawn, collision_detection::Player, movement::Velocity, scheduling::GameSchedule
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<ShootEvent>().add_systems(
      Update,
      (
        do_shooting.in_set(GameSchedule::EntityUpdates),
        do_impact.in_set(GameSchedule::DespawnEntities),
      ),
    );
  }
}

#[derive(Event)]
pub struct ShootEvent {
  pub is_player: bool,
  pub start: Vec3,
  pub velocity: Vec3,
}

impl ShootEvent {
  pub fn new(is_player:bool, start: Vec3, velocity: Vec3) -> Self {
    Self { is_player, start, velocity }
  }
}

#[derive(Component)]
#[require(BoundsDespawn)]
pub struct Bullet {
  pub hit: bool,
  pub damage: f32,
}

fn do_shooting(
  mut commands: Commands,
  mut ev_shoot_events: EventReader<ShootEvent>,
  scene_assets: Res<SceneAssets>,
) {
  for &ShootEvent {is_player, start, velocity } in ev_shoot_events.read() {
//FIXME: yuck
    if is_player{
      commands.spawn((
        Bullet {
          hit: false,
          damage: 20.0,
        },
        Mesh3d(scene_assets.bullet.clone()),
        MeshMaterial3d(scene_assets.bullet_material.clone()),
        Transform::from_translation(start),
        Velocity(velocity),
        Player,
      ));
    }
    else{
      commands.spawn((
        Bullet {
          hit: false,
          damage: 20.0,
        },
        Mesh3d(scene_assets.bullet.clone()),
        MeshMaterial3d(scene_assets.bullet_material.clone()),
        Transform::from_translation(start),
        Velocity(velocity),

      ));
    }
  }
}

fn do_impact(mut commands: Commands, query: Query<(Entity, &Bullet)>) {
  for (entity, bullet) in query.iter() {
    if bullet.hit {
      //play a sound or something
      commands.entity(entity).despawn_recursive();
    }
  }
}
