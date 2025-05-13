use bevy::prelude::*;

use crate::{
  bullet::{Bullet, BulletHitEvent},
  health::HealthEvent,
  hook::Hook,
  scheduling::GameSchedule,
  ship::Invincible,
};

pub struct CollsionDetectionPlugin;

impl Plugin for CollsionDetectionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        PostUpdate,
        (
          player_bullet_collision_detection,
          enemy_bullet_collision_detection,
          player_collision_detection,
        )
          .chain()
          .in_set(GameSchedule::CollisionDetection),
      )
      .add_event::<CollisionEvent>();
    //.add_event::<BulletCollisionEvent>();
  }
}

#[derive(Component)]
pub struct Collider {
  pub radius: f32,
  pub collision_damage: f32,
}

impl Collider {
  pub fn new(radius: f32, collision_damage: f32) -> Self {
    Self {
      radius,
      collision_damage,
    }
  }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Event)]
pub struct CollisionEvent {
  pub player: Entity,
  pub other: Entity,
}

impl CollisionEvent {
  pub fn new(entity: Entity, collided: Entity) -> Self {
    Self {
      player: entity,
      other: collided,
    }
  }
}
/*
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
 */
fn player_bullet_collision_detection(
  mut ev_health_writer: EventWriter<HealthEvent>,
  mut ev_bullet_hit_writer: EventWriter<BulletHitEvent>,
  bullet_query: Query<(Entity, &GlobalTransform, &Bullet), With<Player>>,
  target_query: Query<(Entity, &GlobalTransform, &Collider), Without<Player>>,
) {
  for (bullet_entity, bullet_transform, bullet) in bullet_query.iter() {
    for (target_entity, tagret_transform, collider) in target_query.iter() {
      let dist_sqr = bullet_transform
        .translation()
        .distance_squared(tagret_transform.translation());
      if dist_sqr < collider.radius * collider.radius {
        ev_health_writer.write(HealthEvent::new(target_entity, bullet.owner, bullet.damage));
        ev_bullet_hit_writer.write(BulletHitEvent::new(bullet_entity, Some(target_entity)));
      }
    }
  }
}

fn enemy_bullet_collision_detection(
  mut ev_health_writer: EventWriter<HealthEvent>,
  mut ev_bullet_hit_writer: EventWriter<BulletHitEvent>,
  bullet_query: Query<(Entity, &GlobalTransform, &Bullet), Without<Player>>,
  target_query: Query<
    (Entity, &GlobalTransform, &Collider),
    (With<Player>, Without<Hook>, Without<Invincible>),
  >,
) {
  for (target_entity, tagret_transform, collider) in target_query.iter() {
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter() {
      let dist_sqr = bullet_transform
        .translation()
        .distance_squared(tagret_transform.translation());
      if dist_sqr < collider.radius * collider.radius {
        info!("hit ent {:?}", target_entity);
        ev_health_writer.write(HealthEvent::new(target_entity, bullet.owner, bullet.damage));
        ev_bullet_hit_writer.write(BulletHitEvent::new(bullet_entity, Some(target_entity)));
      }
    }
  }
}

fn player_collision_detection(
  mut ev_health_writer: EventWriter<HealthEvent>,
  mut ev_collision_writer: EventWriter<CollisionEvent>,
  player_query: Query<(Entity, &GlobalTransform, &Collider), (With<Player>, Without<Invincible>)>,
  enemy_query: Query<(Entity, &GlobalTransform, &Collider), Without<Player>>,
) {
  for (player, player_transform, player_collider) in player_query.iter() {
    for (enemy, enemy_transform, enemy_collider) in enemy_query.iter() {
      let dist_sqr = player_transform
        .translation()
        .distance_squared(enemy_transform.translation());
      let collision_seperation = player_collider.radius + enemy_collider.radius;
      if dist_sqr < collision_seperation * collision_seperation {
        ev_health_writer.write(HealthEvent::new(player, Some(enemy), enemy_collider.collision_damage));
        ev_health_writer.write(HealthEvent::new(enemy, Some(player), player_collider.collision_damage));
        ev_collision_writer.write(CollisionEvent::new(player, enemy));
      }
    }
  }
}

/*
fn apply_collisions(
  mut ev_collision_reader: EventReader<CollisionEvent>,
  mut health: Query<&mut Health>,

) {
  for &CollisionEvent { player, other, damage } in ev_collision_reader.read() {
    let Ok(mut player_health) = health.get_mut(player) else {
      continue;
    };
    player_health.value -= damage;
    //set_damaged(player_health);
    //player_health.set_damaged();

    let Ok(mut enemy_health) = health.get_mut(other) else {
      continue;
    };
    enemy_health.value -= 100000.0;
  }
}
 */

/*
fn apply_bullet_collisions(
  mut commands:Commands,
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
    health.value -= bullet_details.damage;
    commands.entity(entity).insert(HitMarker::new());
  }
}
 */
