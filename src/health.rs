use bevy::{color::palettes::css::WHITE, prelude::*, scene::ron::de, text::cosmic_text::ttf_parser::RgbaColor};

use crate::scheduling::GameSchedule;

const HIT_MARKER_COLOUR:Hsla = Hsla::new(40., 0.2, 0.95, 1.0);
pub struct HealthPlugin;

impl Plugin for HealthPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_health)
      .add_systems(Update, apply_health_changes.in_set(GameSchedule::PreDespawnEntities))
      .add_event::<HealthEvent>();

      //.add_observer(hit_marker_observer);
  }
}

#[derive(Event)]
pub struct HealthEvent{
  entity:Entity,
  health_adjustment:f32,
}

impl HealthEvent{
  pub fn new (entity:Entity, health_adjustment:f32)->Self{
    Self{ entity, health_adjustment}
  }
}


#[derive(Component, Default, Clone)]
pub struct Health{
  pub value:f32,
  timer:Timer,
  hit_marker:bool,
}

impl Health{
  pub fn new(value:f32) -> Self{
    Self{
      value:value,
      timer:Timer::from_seconds(0.1, TimerMode::Once),
      hit_marker:false,
    }
  }
}



fn apply_health_changes(mut ev_health_reader:EventReader<HealthEvent>, mut query:Query<&mut Health>){
  for HealthEvent{ entity,  health_adjustment } in ev_health_reader.read(){
    let Ok( mut health ) = query.get_mut(*entity) else {
      continue;
    };
    health.value += health_adjustment;
    if *health_adjustment < 0.{
      health.hit_marker = true;
      health.timer.reset();
    }
  }
}

#[derive(Resource)]
struct HitRegMaterial(Handle<StandardMaterial>);

fn init_health(mut commands:Commands, mut materials: ResMut<Assets<StandardMaterial>>){
  let material = materials.add(StandardMaterial{
    base_color:Color::Hsla(HIT_MARKER_COLOUR),
    emissive: LinearRgba::WHITE,
    ..default()
  });
  commands.insert_resource(HitRegMaterial(material));
}
