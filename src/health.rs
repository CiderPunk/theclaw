use bevy::prelude::*;

use crate::scheduling::GameSchedule;


pub struct HealthPlugin;

impl Plugin for HealthPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, apply_health_changes.in_set(GameSchedule::HealthAdjust))
      .add_event::<HealthEvent>();

  }
}

#[derive(Event)]
pub struct HealthEvent{
  pub entity:Entity,
  pub health_adjustment:f32,
}

impl HealthEvent{
  pub fn new (entity:Entity, health_adjustment:f32)->Self{
    Self{ entity, health_adjustment}
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


fn apply_health_changes(
  mut ev_health_reader:EventReader<HealthEvent>,
  mut query:Query<&mut Health>,
){
  for HealthEvent{ entity,  health_adjustment } in ev_health_reader.read(){
    let Ok( mut health ) = query.get_mut(*entity) else {
      continue;
    };
    health.value += health_adjustment;
  }
}

