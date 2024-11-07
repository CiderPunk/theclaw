use bevy::prelude::*;


#[derive(Component)]
pub struct Velocity{
  pub value: Vec3,
}

impl Velocity{
  pub fn new(value: Vec3) -> Self{
    Self{ value }
  }
}

#[derive(Component)]
pub struct MaxSpeed{
  pub value:f32,
}

impl MaxSpeed{
  pub fn new(value:f32) -> Self{
    Self{ value }
  }
}


#[derive(Component)]
pub struct Acceleration{
  pub value:Vec3,
}

impl Acceleration{
  pub fn new(value:Vec3) -> Self{
    Self{ value }
  }
}

#[derive(Bundle)]
pub struct MovingObjectBundle{
  pub velocity:Velocity,
  pub model: SceneBundle,
}
#[derive(Bundle)]
pub struct AcceleratingObjectBundle{
  pub acceleration:Acceleration,
  pub velocity:Velocity,
  pub model: SceneBundle,
  pub max_speed:MaxSpeed,
}



pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
      app.add_systems(Update, (update_velocity, update_position).chain());
    }
}

fn update_velocity(mut query:Query<(&Acceleration, &MaxSpeed, &mut Velocity)>, time: Res<Time>){
  for (acceleraation, max_speed, mut velocity) in query.iter_mut(){
    velocity.value += acceleraation.value * time.delta_seconds();
    if velocity.value.length() > max_speed.value{
      velocity.value = velocity.value.normalize() * max_speed.value;
    }
  }
}

fn update_position(mut query:Query<(&Velocity, &mut Transform)>, time: Res<Time>){
  for (vel, mut transform) in query.iter_mut(){
    transform.translation += vel.value * time.delta_seconds();
  }
}
