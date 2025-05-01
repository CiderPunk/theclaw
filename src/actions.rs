use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{movement::{Acceleration, Velocity}, scheduling::GameSchedule, ship::PlayerShip};

pub struct ActionPlugin;

impl Plugin for ActionPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (do_drift, do_player_proximity_test, do_track_to_target ).in_set(GameSchedule::EntityUpdates));
  }
}



#[derive(Component)]
pub struct TrackToTarget{
  target:Entity,
  linear_acceleration:f32,
  update_timer:Timer
}

impl TrackToTarget{
  pub fn new( target:Entity, linear_acceleration:f32, update_secs:f32)->Self{
    Self{ target, linear_acceleration, update_timer:Timer::from_seconds(update_secs, TimerMode::Repeating) }
  }
}

fn do_track_to_target(
  mut query:Query<(&mut TrackToTarget, &GlobalTransform, &mut Acceleration)>,
 target_query:Query<&GlobalTransform>, 
 time:Res<Time>
){
  for (mut track_to_target, transform, mut acceleration) in query.iter_mut(){
    track_to_target.update_timer.tick(time.delta());
    if track_to_target.update_timer.just_finished(){
      let Ok(target_transform) = target_query.get(track_to_target.target) else{ continue; } ;
      let diff = (target_transform.translation() - transform.translation()).normalize();
      acceleration.acceleration = diff * track_to_target.linear_acceleration;
    }
  }
}


#[derive(Component)]
pub struct PlayerProximityTest{
  trigger_distance_squared:f32,
  test_timer:Timer,
  action:fn(&mut Commands, Entity, Entity),
}

impl PlayerProximityTest{
  pub fn new ( trigger_distance_squared:f32, test_frequency_seconds:f32, action:fn(&mut Commands, Entity, Entity) )->Self{
    Self{ 
      trigger_distance_squared,
      test_timer:Timer::from_seconds(test_frequency_seconds, TimerMode::Repeating), 
      action,
    }
  }
}

fn do_player_proximity_test(
  mut commands:Commands, 
  mut query:Query<(&mut PlayerProximityTest, &GlobalTransform, Entity)>,
  player_query:Query<(&GlobalTransform, Entity), With<PlayerShip>>,
  time:Res<Time>,
){
  for (mut proximity_test, transform, entity) in query.iter_mut(){
    proximity_test.test_timer.tick(time.delta());
    if proximity_test.test_timer.just_finished(){
      for (player_transform, player) in player_query{
        let diff = transform.translation() - player_transform.translation();
        if diff.length_squared() < proximity_test.trigger_distance_squared{
          (proximity_test.action)(&mut commands, entity, player);
        }
      }
    }
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


