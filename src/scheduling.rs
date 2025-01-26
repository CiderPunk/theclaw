use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone,  PartialEq, Eq, Hash)]
pub enum GameSchedule{
  UserInput,
  EntityUpdates,
  BoundsCheck,
  CollisionDetection, 
  DespawnEntities,
}


pub struct SchedulingPlugin;

impl Plugin for SchedulingPlugin{
  fn build(&self, app: &mut App) {
      app.configure_sets(Update, (
        GameSchedule::DespawnEntities,
        GameSchedule::UserInput,
        GameSchedule::EntityUpdates,
        GameSchedule::BoundsCheck,
        GameSchedule::CollisionDetection
      ).chain());
  }

}
