use bevy::prelude::*;

use crate::scheduling::GameSchedule;

pub struct ActionPlugin;

impl Plugin for ActionPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (do_drift).in_set(GameSchedule::EntityUpdates));
  }
}


fn do_drift(query:Query<Actions::Drift>){


  
}

#[derive(Component)]
enum Actions{
  Home{
    max_speed:f32,
    acceleration:f32,
  },
  Drift{
    variance:Vec3,
    trend:Vec3,
  }
}


