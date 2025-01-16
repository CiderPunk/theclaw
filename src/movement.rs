use bevy::prelude::*;


#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct TargetVelocity(Vec3);

#[derive(Component)]
struct MaxSpeed(f32);

#[derive(Component)]
struct MaxLinearAcceleration(f32);

#[derive(Component)]
struct Acceleration(Vec3);


#[derive(Bundle)]
pub struct MovingObjectBundle{
  pub velocity:Velocity,
  pub transform:Transform,
}
#[derive(Bundle)]
pub struct AcceleratingObjectBundle{
  pub acceleration:Acceleration,
  pub velocity:Velocity,
  pub max_speed:MaxSpeed,
}

#[derive(Bundle)]
pub struct TargetVelocityObjectBundle{
  pub velocity:Velocity,
  pub target_volcity:TargetVelocity,
  pub max_accelleration:MaxLinearAcceleration,
}



pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
      app.add_systems(Update, (update_velocity, update_target_velocity, update_position).chain());
    }
}


fn update_target_velocity(mut query:Query<(&MaxLinearAcceleration, &TargetVelocity, &mut Velocity)>, time: Res<Time>){
  for (max_accelleration, target_velocity, mut velocity) in query.iter_mut(){
    let diff =  target_velocity.value - velocity.value;
    if diff == Vec3::ZERO || diff.try_normalize() == None{
      velocity.value = target_velocity.value;
      return;
    }
    velocity.value = velocity.value + (diff * max_accelleration.value * time.delta_secs());
    if velocity.value.length_squared() > target_velocity.value.length_squared(){
      velocity.value = target_velocity.value;
    }
  }
}



fn update_velocity(mut query:Query<(&Acceleration, &MaxSpeed, &mut Velocity)>, time: Res<Time>){
  for (acceleraation, max_speed, mut velocity) in query.iter_mut(){
    velocity.value += acceleraation.value * time.delta_secs();
    if velocity.value.length() > max_speed.value{
      velocity.value = velocity.value.normalize() * max_speed.value;
    }
  }
}

fn update_position(mut query:Query<(&Velocity, &mut Transform)>, time: Res<Time>){
  for (vel, mut transform) in query.iter_mut(){
    transform.translation += vel.value * time.delta_secs();
  }
}
