use bevy::prelude::*;

use crate::{bullet::Bullet, health::Health, hook::Hook, scheduling::GameSchedule};

pub struct CollsionDetectionPlugin;

impl Plugin for CollsionDetectionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Update,
        (
          player_bullet_collision_detection,
          enemy_bullet_collision_detection,
          player_collision_detection,
          apply_bullet_collisions,
        )
          .chain()
          .in_set(GameSchedule::CollisionDetection),
      )
      .add_event::<CollisionEvent>()
      .add_event::<BulletCollisionEvent>();
  }
}

#[derive(Component)]
pub struct Collider {
  pub radius: f32,
}

impl Collider {
  pub fn new(radius: f32) -> Self {
    Self { radius }
  }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Event)]
pub struct CollisionEvent {
  pub entity: Entity,
  pub collided: Entity,
}

impl CollisionEvent {
  pub fn new(entity: Entity, collided: Entity) -> Self {
    Self { entity, collided }
  }
}

#[derive(Event)]
pub struct BulletCollisionEvent {
  pub entity: Entity,
  pub bullet: Entity,
}

impl BulletCollisionEvent {
  pub fn new(entity: Entity, bullet: Entity) -> Self {
    Self { entity, bullet }
  }
}

fn player_bullet_collision_detection(
  mut ev_bullet_collision: EventWriter<BulletCollisionEvent>,
  bullet_query: Query<(Entity, &GlobalTransform), (With<Bullet>, With<Player>)>,
  target_query: Query<(Entity, &GlobalTransform, &Collider), Without<Player>>,
) {
  for (bullet, bullet_transform) in bullet_query.iter() {
    for (target, tagret_transform, collider) in target_query.iter() {
      let dist_sqr = bullet_transform
        .translation()
        .distance_squared(tagret_transform.translation());
      if dist_sqr < collider.radius * collider.radius {
        ev_bullet_collision.send(BulletCollisionEvent::new(target, bullet));
      }
    }
  }
}

fn enemy_bullet_collision_detection(
  mut ev_bullet_collision: EventWriter<BulletCollisionEvent>,
  bullet_query: Query<(Entity, &GlobalTransform), (With<Bullet>, Without<Player>)>,
  target_query: Query<(Entity, &GlobalTransform, &Collider), (With<Player>, Without<Hook>)>,
) {
  for (target, tagret_transform, collider) in target_query.iter() {
    for (bullet, bullet_transform) in bullet_query.iter() {
      let dist_sqr = bullet_transform
        .translation()
        .distance_squared(tagret_transform.translation());
      if dist_sqr < collider.radius * collider.radius {
        ev_bullet_collision.send(BulletCollisionEvent::new(target, bullet));
      }
    }
  }
}

fn player_collision_detection(
  mut ev_collision: EventWriter<CollisionEvent>,
  player_query: Query<(Entity, &GlobalTransform, &Collider), With<Player>>,
  enemy_query: Query<(Entity, &GlobalTransform, &Collider), Without<Player>>,
) {
  for (player, player_transform, player_collider) in player_query.iter() {
    for (enemy, enemy_transform, enemy_collider) in enemy_query.iter() {
      //newly spawned ents have 0.,0.,0. always! ANNOYING
      //if player_transform.translation() == Vec3::ZERO || enemy_transform.translation() == Vec3::ZERO{
      //  continue;
      //}
      let dist_sqr = player_transform
        .translation()
        .distance_squared(enemy_transform.translation());
      let collision_seperation = player_collider.radius + enemy_collider.radius;
      if dist_sqr < collision_seperation * collision_seperation {
        info!("Collision detected! dist: {:?},  player:{:?}, other: {:?} player pos: {:?} other pos: {:?}", dist_sqr, player, enemy, player_transform.translation(), enemy_transform.translation());
        ev_collision.send(CollisionEvent::new(player, enemy));
      }
    }
  }
}

fn apply_bullet_collisions(
  mut ev_bullet_collision: EventReader<BulletCollisionEvent>,
  mut health_query: Query<&mut Health>,
  mut bullet_query: Query<&mut Bullet>,
) {
  for &BulletCollisionEvent { entity, bullet } in ev_bullet_collision.read() {
    let Ok(mut bullet_details) = bullet_query.get_mut(bullet) else {
      continue;
    };
    bullet_details.hit = true;

    let Ok(mut health) = health_query.get_mut(entity) else {
      continue;
    };
    health.0 -= bullet_details.damage;
  }
}
