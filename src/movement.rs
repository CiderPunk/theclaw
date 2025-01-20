use bevy::prelude::*;

pub struct MovementPlugin;

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
      app.add_systems(Update, update_position);
    }
}

fn update_position(mut query:Query<(&mut Transform, &Velocity)>, time: Res<Time>){
  for (mut transform, velocity) in &mut query{
    transform.translation.x += velocity.x * time.delta_secs();
    transform.translation.z += velocity.y * time.delta_secs();
  }
}


