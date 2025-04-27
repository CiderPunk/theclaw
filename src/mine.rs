use bevy::prelude::*;
use rand::Rng;

use crate::{asset_loader::SceneAssets, bounds_check::BoundsDespawn, collision_detection::Collider, effect_sprite::{EffectSpriteEvent, EffectSpriteType}, enemy::{Enemy, ENEMY_START_POINT_X, ENEMY_START_POINT_Z_BOUNDS_MAX}, game_manager::PointEvent, health::Health, hit_marker::HitMarker, hook::Hookable, movement::{Roller, Velocity}, scheduling::GameSchedule};


const MINE_SPAWN_TIME_SECONDS: f32 = 3.0;
const MINE_COLLISION_RADIUS: f32 = 1.6;
const MINE_COLLISION_DAMAGE: f32 = -30.;
const MINE_HEALTH:f32 = 10.;
const MINE_SPIN_SPEED:f32 = 1.2;
const MINE_HOOK_TRANSLATION: Vec3 = Vec3::new(-1., 0., 0.);
const MINE_HOOK_ROTATION: f32 = 0.;
const MINE_NAME: &str = "mine";

const MINE_POINTS:u64 = 20;
pub struct MinePlugin;

impl Plugin for MinePlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (spawn_mine ).in_set(GameSchedule::EntityUpdates))
    .add_systems(Update, check_dead.in_set(GameSchedule::DespawnEntities));
  }
}

#[derive(Deref, DerefMut)]
pub struct SpawnTimer(Timer);

impl Default for SpawnTimer {

fn default() -> Self {
    Self(Timer::from_seconds(
      MINE_SPAWN_TIME_SECONDS,
      TimerMode::Repeating,
    ))
  }
}

#[derive(Component)]
#[require(Enemy, BoundsDespawn, Hookable, HitMarker)]
struct Mine;



fn spawn_mine(
  mut commands:Commands,
  mut spawn_timer:Local<SpawnTimer>,
  time:Res<Time>,
  scene_assets:Res<SceneAssets>,
){
  spawn_timer.tick(time.delta());
  if !spawn_timer.just_finished(){ 
    return;
  }

  let mut rng = rand::thread_rng();
  let spawn_pos = rng.gen_range(-1. ..1.);
  let start_z = ENEMY_START_POINT_Z_BOUNDS_MAX * spawn_pos;
  //ai_data_collection.get.get_behaviour_handle(MINE_NAME, "" )

commands.spawn((
    Mine,
    SceneRoot(scene_assets.mine.clone()),
    Transform::from_translation(Vec3::new(ENEMY_START_POINT_X, 0., start_z)),
    Velocity(Vec3::new(10., 0., 0.)),
    Collider{
      radius: MINE_COLLISION_RADIUS,
      collision_damage: MINE_COLLISION_DAMAGE,
    },
    Hookable::new(
      MINE_HOOK_TRANSLATION,
      Quat::from_rotation_z(MINE_HOOK_ROTATION),
    ),
    Health::new(MINE_HEALTH),
    Roller::new(0.2, 0.1, MINE_SPIN_SPEED),
  ));
}

fn check_dead(
  mut commands: Commands,
  query: Query<(Entity, &Health, &GlobalTransform, &Velocity), With<Mine>>,
  mut ev_point_writer: EventWriter<PointEvent>,
  mut ev_splosion_writer: EventWriter<EffectSpriteEvent>,
) {
  for (entity, health, transform, velocity) in query.iter() {
    if health.value <= 0. {
      info!("dead {:?}", entity);
      ev_splosion_writer.write( 
        EffectSpriteEvent::new(  
        transform.translation() + Vec3::new(0., 0., 0.),
        3.0,
        velocity.0,
        EffectSpriteType::Splosion, 
      ));
      

      commands.entity(entity).despawn();
      ev_point_writer.write(PointEvent(MINE_POINTS));
    }
  }
}

