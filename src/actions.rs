use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

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

impl Drift{
  pub fn new(variance:Vec3, trend:Vec3, update_secs:f32)->Self{
    Self{ variance, trend, update_timer:Timer::from_seconds(update_secs, TimerMode::Repeating)}
  }
}

fn do_drift(mut query:Query<(&mut Drift, &mut Acceleration)>, time:Res<Time>){
  let mut rng = rand::thread_rng();
  for (mut drift, mut acceleration) in query.iter_mut(){
    drift.update_timer.tick(time.delta());
    if drift.update_timer.just_finished(){
      acceleration.acceleration = (rng.gen_range(-1. .. 1.) * drift.variance) + drift.trend;
    }
  }
}


