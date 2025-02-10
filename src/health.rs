use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Health(pub f32);

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
  fn build(&self, _app: &mut App) {}
}
