use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin{
  fn build(&self, app: &mut App) {
    app.add_observer(hit_marker_observer);
  }
}

#[derive(Component, Default, Clone)]
pub struct Health{
  pub value:f32,
}

impl Health{
  pub fn new(value:f32) -> Self{
    Self{
      value:value,
    }
  }
}


#[derive(Component)]
pub struct HitMarker{
  timer:Timer,
}

impl HitMarker{
  pub fn new()->Self{
    Self{ timer:Timer::from_seconds(0.1, TimerMode::Once) }
  }
}

fn hit_marker_observer(trigger:Trigger<OnAdd, HitMarker>){
  info!("Hitmarker for {:?}", trigger.entity());

}
