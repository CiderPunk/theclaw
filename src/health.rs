use bevy::{ecs::system::command, prelude::*};

use crate::scheduling::GameSchedule;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Update,
        (apply_health_changes, make_dead).chain().in_set(GameSchedule::HealthAdjust),
      )
      .add_event::<HealthEvent>();
  }
}


#[derive(Component)]
pub struct Dead{
  pub killer:Option<Entity>
}

#[derive(Event)]
pub struct HealthEvent {
  pub entity: Entity,
  pub inflictor: Option<Entity>,
  pub health_adjustment: f32,
}

impl HealthEvent {
  pub fn new(entity: Entity, inflictor:Option<Entity>,  health_adjustment: f32) -> Self {
    Self {
      entity,
      inflictor,
      health_adjustment,
    }
  }
}

#[derive(Component, Default, Clone)]
pub struct Health {
  pub value: f32,
  pub max: f32,
  last_hurt_by:Option<Entity>,
}

impl Health {
  pub fn new(value: f32) -> Self {
    Self { value, max: value, last_hurt_by:None, }
  }
}

fn apply_health_changes(
  mut ev_health_reader: EventReader<HealthEvent>,
  mut query: Query<&mut Health>,
) {
  for HealthEvent {
    entity,
    inflictor,
    health_adjustment,
  } in ev_health_reader.read()
  {
    let Ok(mut health) = query.get_mut(*entity) else {
      continue;
    };
    if health.value >= 0.{
      if *health_adjustment < 0. && inflictor.is_some(){
        health.last_hurt_by = inflictor.clone();
      }
      health.value = (health.value + health_adjustment).min(health.max);
    }
  }
}

fn make_dead(mut commands:Commands, query:Query<(Entity, &Health)>){
  for (entity, health) in query {
    if health.value <= 0.{
      commands.entity(entity).insert(
        Dead{
          killer:health.last_hurt_by,
        });
    }
  }
}
