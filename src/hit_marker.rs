use bevy::prelude::*;

use crate::{
  health::{Health, HealthEvent},
  scheduling::GameSchedule,
};

const HIT_MARKER_COLOUR: Hsla = Hsla::new(40., 0.2, 0.95, 1.0);
//const HIT_MARKER_TIME: f32 = 0.1666;
const HIT_MARKER_TIME: f32 = 0.0833;

pub struct HitMarkerPlugin;

impl Plugin for HitMarkerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_hit_marker).add_systems(
      Update,
      (apply_hit_marker, update_hit_markers)
        .chain()
        .in_set(GameSchedule::EntityUpdates),
    );
  }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct OriginalMaterial {
  handle: Handle<StandardMaterial>,
}

#[derive(Component, Clone)]
pub struct HitMarker {
  timer: Timer,
  active: bool,
}

impl Default for HitMarker {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(HIT_MARKER_TIME, TimerMode::Once),
      active: false,
    }
  }
}

#[derive(Resource)]
struct HitMarkerMaterial(Handle<StandardMaterial>);

fn init_hit_marker(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
  let material = materials.add(StandardMaterial {
    base_color: Color::Hsla(HIT_MARKER_COLOUR),
    emissive: LinearRgba::WHITE,
    ..default()
  });
  commands.insert_resource(HitMarkerMaterial(material));
}

fn apply_hit_marker(
  mut commands: Commands,
  mut ev_health_reader: EventReader<HealthEvent>,
  mut query: Query<(&mut HitMarker, &Health)>,
  children: Query<&Children>,
  mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
  hit_material: Res<HitMarkerMaterial>,
  original_material_query: Query<&OriginalMaterial>,
) {
  for HealthEvent {
    entity,
    inflictor,
    health_adjustment,
  } in ev_health_reader.read()
  {
    let Ok((mut hit_marker, health)) = query.get_mut(*entity) else {
      continue;
    };

    if *health_adjustment < 0. && health.value > 0. {
      let mut found_material = false;
      for descendant in children.iter_descendants(*entity) {
        if let Ok(material) = mesh_materials.get(descendant) {
          let original_material = original_material_query.get(descendant);
          commands
            .entity(descendant)
            .insert(MeshMaterial3d(hit_material.0.clone()))
            .insert_if_new_and(
              OriginalMaterial {
                handle: material.clone_weak(),
              },
              || original_material.is_err(),
            );

          found_material = true;
        }
        /*
        else{
          commands
            .entity(descendant)
            .insert(MeshMaterial3d(hit_material.0.clone()));

        }
         */
      }
      if found_material {
        //info!("adding hitmarker for {:?}",entity);
        hit_marker.active = true;
        hit_marker.timer.reset();
      }
    }
  }
}

fn update_hit_markers(
  mut commands: Commands,
  mut query: Query<(Entity, &mut HitMarker)>,
  children: Query<&Children>,
  original_material_query: Query<&OriginalMaterial>,
  time: Res<Time>,
) {
  for (entity, mut hit_marker) in query.iter_mut() {
    if hit_marker.active {
      hit_marker.timer.tick(time.delta());
      if hit_marker.timer.just_finished() {
        hit_marker.active = false;

        //info!("removing hitmarker for {:?}",entity);
        for descendant in children.iter_descendants(entity) {
          if let Ok(original_material) = original_material_query.get(descendant) {
            commands
              .entity(descendant)
              .insert(MeshMaterial3d(original_material.handle.clone()));
          }
        }
      }
    }
  }
}
