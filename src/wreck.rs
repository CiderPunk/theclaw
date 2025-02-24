use bevy::prelude::*;

pub struct WreckPlugin;

impl Plugin for WreckPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_wreck);
  }
}


#[derive(Resource)]
struct WreckMaterial(Handle<StandardMaterial>);


fn init_wreck(
  mut commands:Commands, 
  asset_server: Res<AssetServer>, 
  mut materials: ResMut<Assets<StandardMaterial>>, 
){
  let wreck_texture = asset_server.load("textures/wreck_alpha.png");
  let wreck_material = materials.add(StandardMaterial{ 
    emissive_texture: Some(wreck_texture), 
    ..Default::default()
  });
  commands.insert_resource(WreckMaterial(wreck_material));
}


#[derive(Component)]
pub struct Wreck{
  init:bool,
  time_to_live:Timer,
}

impl Wreck{
  pub fn new(time_to_live:f32 ) ->Self{
    Self{ 
      time_to_live:Timer::from_seconds(time_to_live, TimerMode::Once),
      init:false,
    }
  }
}



