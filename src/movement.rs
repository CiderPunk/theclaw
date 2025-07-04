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

impl Acceleration {
  pub fn new(acceleration: Vec3, damping: f32, max_speed: f32) -> Self {
    Self {
      acceleration,
      damping,
      max_speed,
    }
  }
}

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Update,
        (update_velocity, update_position)
          .chain()
          .in_set(GameSchedule::EntityUpdates),
      )
      .add_systems(Update, update_roll.in_set(GameSchedule::EntityUpdates));
  }
}

#[derive(Component)]
pub struct Roller {
  pub roll:Vec3,
  
}

impl Roller{
  pub fn new(roll:f32, pitch:f32, yaw:f32,)->Self{
    Self{ roll: Vec3 { x: roll, y: pitch, z: yaw } }
  }
}


fn update_roll(mut query: Query<(&mut Transform, &Roller)>, time: Res<Time>) {
  for (mut transform, roller) in query.iter_mut() {

    if roller.roll.x != 0.{
      transform.rotate_local_x(roller.roll.x * time.delta_secs());
    }
    if roller.roll.y != 0.{
      transform.rotate_local_y(roller.roll.y * time.delta_secs());
    }
    if roller.roll.z != 0.{
      transform.rotate_local_z(roller.roll.z * time.delta_secs());
    }

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
