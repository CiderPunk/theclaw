use bevy::prelude::*;

use crate::scheduling::GameSchedule;

pub struct WreckPlugin;

impl Plugin for WreckPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_wreck)
      .add_systems(Update, wreck_check.in_set(GameSchedule::EntityUpdates))
      .add_systems(Update, update_wrecks.in_set(GameSchedule::DespawnEntities))
      .add_event::<WreckedEvent>();
  }
}


#[derive(Event)]
pub struct WreckedEvent{
  entity:Entity,
  time_to_live:f32,
}

impl WreckedEvent{
  pub fn new(entity:Entity, time_to_live: f32) -> Self {
    Self { entity, time_to_live }
  }
}

fn update_wrecks(mut commands:Commands, mut query:Query<(Entity, &mut Wreck)>, time:Res<Time>){
  for (entity, mut wreck) in query.iter_mut(){
    wreck.time_to_live.tick(time.delta());
    if wreck.time_to_live.just_finished(){
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn wreck_check(
  mut commands:Commands,
  mut ev_wrecked_reader:EventReader<WreckedEvent>, 
  children: Query<&Children>,
  wreck_material:Res<WreckMaterial>,
){
  for &WreckedEvent{ entity, time_to_live } in ev_wrecked_reader.read(){
    commands
      .entity(entity)
      .insert_if_new(
        Wreck{ 
          time_to_live:Timer::from_seconds(time_to_live, TimerMode::Once)
        });


    for descendants in children.iter_descendants(entity) {
      commands
        .entity(descendants)
        .insert(MeshMaterial3d(wreck_material.0.clone()));
    }

  }
}


#[derive(Resource)]
struct WreckMaterial(Handle<StandardMaterial>);

fn init_wreck(
  mut commands:Commands, 
  asset_server: Res<AssetServer>, 
  mut materials: ResMut<Assets<StandardMaterial>>, 
){
  let wreck_texture = asset_server.load("textures/wrecked_alpha.png");
  let wreck_material = materials.add(StandardMaterial{ 
    base_color:Color::Hsla(Hsla::WHITE),
    base_color_texture: Some(wreck_texture.clone()),
    emissive:LinearRgba::WHITE,
    emissive_texture: Some(wreck_texture.clone()),
    alpha_mode:AlphaMode::Mask(0.5),
    ..default()
  });
  commands.insert_resource(WreckMaterial(wreck_material));
}




#[derive(Component)]
pub struct Wreck{
  time_to_live:Timer,
}

impl Wreck{
  pub fn new(time_to_live:f32 ) ->Self{
    Self{ 
      time_to_live:Timer::from_seconds(time_to_live, TimerMode::Once),
    }
  }
}



