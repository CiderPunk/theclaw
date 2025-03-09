use bevy::prelude::*;

use crate::scheduling::GameSchedule;

const HIT_MARKER_COLOUR:Hsla = Hsla::new(40., 0.2, 0.95, 1.0);
pub struct HealthPlugin;

impl Plugin for HealthPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_health)
      .add_systems(Update, (
        apply_health_changes.in_set(GameSchedule::PreDespawnEntities),
        update_hit_markers.in_set(GameSchedule::EntityUpdates),
      
      ))
      .add_event::<HealthEvent>();

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


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct OriginalMaterial(Handle<StandardMaterial>);

#[derive(Component, Default, Clone)]
pub struct Health{
  pub value:f32,
  hit_marker_timer:Timer,
  hit_marker:bool,
}

impl Health{
  pub fn new(value:f32) -> Self{
    Self{
      value:value,
      hit_marker_timer:Timer::from_seconds(0.1, TimerMode::Once),
      hit_marker:false,
    }
  }
}

fn update_hit_markers(
  mut commands: Commands,
  mut query:Query<(Entity, &mut Health)>,
  children:Query<&Children>,
  original_material_query:Query<&OriginalMaterial>,
  time:Res<Time>,
){
  for (entity, mut health) in query.iter_mut(){
    if health.hit_marker{
      health.hit_marker_timer.tick(time.delta());
      if health.hit_marker_timer.just_finished(){
        health.hit_marker = false;
        for descendant in children.iter_descendants(entity){
          if let Ok(original_material) = original_material_query.get(descendant){
            commands.entity(descendant).insert(MeshMaterial3d(original_material.0.clone())); 
          }
        }
      }
    }
  }
}

fn apply_health_changes(
  mut commands:Commands,
  mut ev_health_reader:EventReader<HealthEvent>,
  mut query:Query<&mut Health>,
  children:Query<&Children>,
  mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
  hit_material:Res<HitMarkerMaterial>,

){
  for HealthEvent{ entity,  health_adjustment } in ev_health_reader.read(){
    let Ok( mut health ) = query.get_mut(*entity) else {
      continue;
    };
    health.value += health_adjustment;
    //info!("applying health mod {:?} to {:?}, now {:?}", health_adjustment, entity, health.value);
    if *health_adjustment < 0.{
      let mut found_material = false;
      for descendant in children.iter_descendants(*entity){
        if let Some(material) = mesh_materials
          .get(descendant)
          .ok()
        {
          commands.entity(descendant).insert_if_new(OriginalMaterial(material.clone_weak())).insert(MeshMaterial3d(hit_material.0.clone()));
          found_material = true;
        }
      }
      if found_material{
        health.hit_marker = true;
        health.hit_marker_timer.reset();
      }
    }
  }
}

#[derive(Resource)]
struct HitMarkerMaterial(Handle<StandardMaterial>);

fn init_health(mut commands:Commands, mut materials: ResMut<Assets<StandardMaterial>>){
  let material = materials.add(StandardMaterial{
    base_color:Color::Hsla(HIT_MARKER_COLOUR),
    emissive: LinearRgba::WHITE,
    ..default()
  });
  commands.insert_resource(HitMarkerMaterial(material));
}
