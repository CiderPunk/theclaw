use bevy::prelude::*;

use crate::scheduling::GameSchedule;

pub const DESPAWN_X_MAX:f32 = 80.0;
pub const DESPAWN_X_MIN:f32 = -80.0;
pub const DESPAWN_Z_MAX:f32 = 40.0;
pub const DESPAWN_Z_MIN:f32 = -40.0;

pub struct BoundsCheckPlugin;

impl Plugin for BoundsCheckPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, bounds_check.in_set(GameSchedule::DespawnEntities));
  }
}

fn bounds_check(mut commands:Commands, query:Query<(Entity, &GlobalTransform)>){
  for (entity, transform) in query.iter() {
    if transform.translation().x > DESPAWN_X_MAX ||
      transform.translation().x < DESPAWN_X_MIN ||
      transform.translation().z > DESPAWN_Z_MAX ||
      transform.translation().z < DESPAWN_Z_MIN 
    {
      info!("despawning {:?}", entity);
      commands.entity(entity).despawn();
    }
  }
}
