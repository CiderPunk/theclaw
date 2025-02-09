use crate::scheduling::GameSchedule;
use bevy::prelude::*;

pub struct MovementPlugin;

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

const STOPPED_SPEED: f32 = 2.;

#[derive(Component, Default)]
#[require(Velocity)]
pub struct Acceleration {
  pub acceleration: Vec3,
  pub damping: f32,
  pub max_speed: f32,
}

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (update_velocity, update_position)
        .chain()
        .in_set(GameSchedule::EntityUpdates),
    );
  }
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
  for (mut transform, velocity) in &mut query {
    transform.translation += velocity.0 * time.delta_secs();
  }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
  for (mut velocity, acceleration) in &mut query {
    let mut vel = velocity.0;
    let mut acc = acceleration.acceleration;

    if acc == Vec3::ZERO && vel.length_squared() < STOPPED_SPEED {
      velocity.0 = Vec3::ZERO;
      continue;
    }

    //damping
    acc += -vel.normalize_or_zero() * acceleration.damping;
    vel += acc * time.delta_secs();
    if vel.length_squared() > (acceleration.max_speed * acceleration.max_speed) {
      velocity.0 = vel.normalize_or_zero() * acceleration.max_speed;
    } else {
      velocity.0 = vel;
    }
  }
}
