use bevy::prelude::*;
use rand::Rng;
use std::{f32::consts::PI, time::Duration};

use crate::{
  asset_loader::SceneAssets,
  bounds_check::BoundsDespawn,
  bullet::ShootEvent,
  collision_detection::Collider,
  enemy::*,
  health::Health,
  hook::{Hookable, Hooked},
  movement::{Roller, Velocity},
  scheduling::GameSchedule,
  ship::Captured,
  wreck::{Wreck, WreckedEvent},
};

const SIDEWINDER_SPAWN_TIME_SECONDS: f32 = 2.;
const SIDEWINDER_SPIN_SPEED: f32 = 3.0;
const SIDEWINDER_VERTICAL_VARIANCE: f32 = 10.0;
const SIDEWINDER_SHOOT_SPEED: f32 = 16.0;
const SIDEWINDER_COLLISION_RADIUS: f32 = 2.5;

const SIDEWINDER_SHOOT_TIME: f32 = 1.7;
const SIDEWINDER_CAPTURED_SHOOT_TIME: f32 = 0.4;
const SIDEWINDER_CAPTURED_SHOOT_SPEED: f32 = 48.0;
const SIDEWINDER_BLAST_SIZE: f32 = 3.0;

const SIDEWINDER_HOOK_TRANSLATION: Vec3 = Vec3::new(-3., 0., 0.);
const SIDEWINDER_HOOK_ROTATION: f32 = 0.0;

pub struct SidewinderPlugin;

impl Plugin for SidewinderPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Update,
        (spawn_sidewinder, shoot, shoot_captured).in_set(GameSchedule::EntityUpdates),
      )
      .add_systems(Update, check_dead.in_set(GameSchedule::DespawnEntities));
  }
}

#[derive(Deref, DerefMut)]
pub struct SpawnTimer(Timer);

impl Default for SpawnTimer {
  fn default() -> Self {
    Self(Timer::from_seconds(
      SIDEWINDER_SPAWN_TIME_SECONDS,
      TimerMode::Repeating,
    ))
  }
}

#[derive(Component)]
#[require(Enemy, BoundsDespawn, Hookable)]
struct Sidewinder {
  shoot_timer: Timer,
}

fn shoot_captured(
  mut query: Query<(&mut Sidewinder, &GlobalTransform), With<Captured>>,
  //captor_query: Query<&Velocity>,
  time: Res<Time>,
  mut ev_shoot_event_writer: EventWriter<ShootEvent>,
) {
  for (mut sidewinder, transform) in &mut query {
    sidewinder
      .shoot_timer
      .set_duration(Duration::from_secs_f32(SIDEWINDER_CAPTURED_SHOOT_TIME));
    sidewinder.shoot_timer.tick(time.delta());
    if sidewinder.shoot_timer.finished() {
      //info!("Shooting");
      ev_shoot_event_writer.send(ShootEvent::new(
        true,
        transform.translation() + (transform.left() * 3.0),
        transform.left() * SIDEWINDER_CAPTURED_SHOOT_SPEED,
      ));
    }
  }
}

fn shoot(
  mut query: Query<
    (&mut Sidewinder, &GlobalTransform, &Velocity),
    (Without<Hooked>, Without<Captured>),
  >,
  time: Res<Time>,
  mut ev_shoot_event_writer: EventWriter<ShootEvent>,
) {
  for (mut sidewinder, transform, velocity) in &mut query {
    sidewinder.shoot_timer.tick(time.delta());
    if sidewinder.shoot_timer.finished() {
      //info!("Shooting");
      ev_shoot_event_writer.send(ShootEvent::new(
        false,
        transform.translation() + (transform.left() * 3.0),
        velocity.0 + (transform.left() * SIDEWINDER_SHOOT_SPEED),
      ));
    }
  }
}

fn check_dead(
  mut commands: Commands,
  query: Query<(Entity, &Health, &GlobalTransform, &Velocity), (With<Sidewinder>, Without<Wreck>)>,
  mut ev_wreck_writer: EventWriter<WreckedEvent>,
  scene_assets: Res<SceneAssets>,
) {
  for (entity, health, transform, velocity) in query.iter() {
    if health.0 <= 0. {
      info!("dead");
      //   ev_splosion_writer.send(SplosionEvent::new(transform.translation(), 3.0,velocity.0));
      ev_wreck_writer.send(WreckedEvent::new(
        scene_assets.sidewinder.clone(),
        transform.translation(),
        transform.rotation(),
        velocity.0,
        SIDEWINDER_SPIN_SPEED,
        1.5,
        SIDEWINDER_BLAST_SIZE,
      ));
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn spawn_sidewinder(
  mut commands: Commands,
  time: Res<Time>,
  mut timer: Local<SpawnTimer>,
  scene_assets: Res<SceneAssets>,
) {
  timer.tick(time.delta());
  if !timer.just_finished() {
    return;
  }

  let mut rng = rand::thread_rng();
  let spawn_pos = rng.gen_range(-1. ..1.);
  let start_z = ENEMY_START_POINT_Z_BOUNDS_MAX * spawn_pos;
  let vel_z = spawn_pos * -SIDEWINDER_VERTICAL_VARIANCE;

  //info!("Spawn sidewinder");
  commands.spawn((
    Sidewinder {
      shoot_timer: Timer::from_seconds(SIDEWINDER_SHOOT_TIME, TimerMode::Repeating),
    },
    SceneRoot(scene_assets.sidewinder.clone()),
    Transform::from_translation(Vec3::new(ENEMY_START_POINT_X, 0., start_z))
      .with_rotation(Quat::from_rotation_z(PI)),
    Velocity(Vec3::new(20.0, 0., vel_z)),
    Collider {
      radius: SIDEWINDER_COLLISION_RADIUS,
    },
    Hookable::new(
      SIDEWINDER_HOOK_TRANSLATION,
      Quat::from_rotation_z(SIDEWINDER_HOOK_ROTATION),
    ),
    Health(25.0),
    Roller {
      roll_speed: SIDEWINDER_SPIN_SPEED,
    },
  ));
}
