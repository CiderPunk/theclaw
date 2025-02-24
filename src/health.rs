use bevy::prelude::*;

use crate::scheduling::GameSchedule;

#[derive(Component, Default)]
pub struct Health(pub f32);

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
  fn build(&self, app: &mut App) {
   // app.add_systems(Update, health_check.in_set(GameSchedule::DespawnEntities));
  }
}
/*
fn health_check(mut commands:Commands, query:Query<(&Health, Entity)>){
  for (health, entity) in query.iter(){
    if health.0 <0.{
      commands.entity(entity).despawn_recursive();
    }
  }
}
 */