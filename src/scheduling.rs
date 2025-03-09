use bevy::{ecs::schedule::{LogLevel, ScheduleBuildSettings}, prelude::*};

use crate::state::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSchedule {
  UserInput,
  EntityUpdates,
  CollisionDetection,
  DespawnEntities,
  PreDespawnEntities,
  HealthAdjust,
}

pub struct SchedulingPlugin;

impl Plugin for SchedulingPlugin {
  fn build(&self, app: &mut App) {
    app
      .configure_sets(
        Update,
        (
          GameSchedule::HealthAdjust,
          GameSchedule::PreDespawnEntities,
          GameSchedule::DespawnEntities,
          GameSchedule::UserInput,
          GameSchedule::EntityUpdates,
        )
          .chain()
          .run_if(in_state(GameState::Playing)),
      )
      .configure_sets(
        PostUpdate,
        GameSchedule::CollisionDetection
          .after(TransformSystem::TransformPropagate)
          .run_if(in_state(GameState::Playing)),
      );
      
    app.edit_schedule(Update, |schedule| {
      schedule.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        ..default()
      });
    });
     
  }
}
