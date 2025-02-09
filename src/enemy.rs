use bevy::prelude::*;

pub const ENEMY_START_POINT_X: f32 = -70.0;
//pub const ENEMY_START_POINT_Z_BOUNDS_MIN:f32 = -26.0;
pub const ENEMY_START_POINT_Z_BOUNDS_MAX: f32 = 26.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, _app: &mut App) {
    //   app.add_systems(Update, bounds_check.in_set(GameSchedule::DespawnEntities));
  }
}

#[derive(Component, Default)]
pub struct Enemy;
