use bevy::prelude::*;

use crate::{movement::{Acceleration, Velocity}, scheduling::GameSchedule};

pub struct ActionPlugin;

impl Plugin for ActionPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (do_drift).in_set(GameSchedule::EntityUpdates));
  }
}




#[derive(Component)]
#[require(Acceleration, Velocity)]
pub struct Drift{
  variance:Vec3,
  trend:Vec3,
  update_timer:Timer,
}

fn do_drift(mut query:Query<(&mut Drift, &mut Acceleration)>){
  for (drift, acceleration) in query.iter_mut(){


  }
}


