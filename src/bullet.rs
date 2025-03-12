use bevy::{math::VectorSpace, prelude::*};

use crate::{
  asset_loader::SceneAssets, bounds_check::BoundsDespawn, collision_detection::Player, effect_sprite::{EffectSpriteEvent, EffectSpriteType}, movement::Velocity, scheduling::GameSchedule
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ShootEvent>()
      .add_event::<BulletHitEvent>()
      .add_systems(
        Update,
        (
          do_shooting.in_set(GameSchedule::EntityUpdates),
          bullet_hits.in_set(GameSchedule::DespawnEntities),
        ),
      );
  }
}

#[derive(Event)]
pub struct BulletHitEvent {
  bullet: Entity,
  other: Option<Entity>,
}

impl BulletHitEvent {
  pub fn new(entity: Entity, other:Option<Entity>) -> Self {
    Self { bullet: entity, other, }
  }
}

#[derive(Event)]
pub struct ShootEvent {
  pub is_player: bool,
  pub start: Vec3,
  pub velocity: Vec3,
  pub damage: f32,
}

impl ShootEvent {
  pub fn new(is_player: bool, start: Vec3, velocity: Vec3, damage: f32) -> Self {
    Self {
      is_player,
      start,
      velocity,
      damage,
    }
  }
}

#[derive(Component)]
#[require(BoundsDespawn)]
pub struct Bullet {
  //pub hit: bool,
  pub damage: f32,
}

fn do_shooting(
  mut commands: Commands,
  mut ev_shoot_events: EventReader<ShootEvent>,
  scene_assets: Res<SceneAssets>,
) {
  for &ShootEvent {
    is_player,
    start,
    velocity,
    damage,
  } in ev_shoot_events.read()
  {
    //FIXME: yuck
    if is_player {
      commands.spawn((
        Bullet { damage },
        Mesh3d(scene_assets.bullet.clone()),
        MeshMaterial3d(scene_assets.bullet_material.clone()),
        Transform::from_translation(start),
        Velocity(velocity),
        Player,
      ));
    } else {
      commands.spawn((
        Bullet { damage },
        Mesh3d(scene_assets.bullet.clone()),
        MeshMaterial3d(scene_assets.bullet_material.clone()),
        Transform::from_translation(start),
        Velocity(velocity),
      ));
    }
  }
}

fn bullet_hits(
  mut commands: Commands, 
  mut ev_bullet_hit_reader: EventReader<BulletHitEvent>,
  bullet_query: Query<&GlobalTransform>,
  target_query: Query<&Velocity>,
  mut ev_effect_sprite_writer: EventWriter<EffectSpriteEvent>,
) {
  for hit_event in ev_bullet_hit_reader.read() {
    
    if let Ok(transform) = bullet_query.get(hit_event.bullet){
      let mut velocity = Vec3::ZERO;
      if hit_event.other.is_some(){

        if let Ok(target_velocity) = target_query.get(hit_event.other.unwrap()){
          velocity = target_velocity.0;
        }
      }

      ev_effect_sprite_writer.send(EffectSpriteEvent::new(transform.translation(), 1., velocity, EffectSpriteType::Ricochet));
    }
    commands.entity(hit_event.bullet).despawn_recursive();

    //TODO: spawn hit effect / sound
  }
}
