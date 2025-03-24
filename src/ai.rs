use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::{asset_loader::AssetsLoading, state::GameState};
pub struct AiPlugin;

impl Plugin for AiPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(JsonAssetPlugin::<AiConfig>::new(&["aiconfig.json"]),)
      .init_resource::<AiData>()
      .add_systems(Startup, load_configs)
      .add_systems(OnExit(GameState::Loading), parse_configs);
    //app
    //.init_resource::<BehaviourCollection>();
  }
}

#[derive(Component)]
pub struct AiRegister{
  pub name:String,
  pub index:Option<usize>,
}

impl AiRegister{
  pub fn new(name:&str)->Self{
    Self{ name:name.to_string(), index:None }
  }
}


fn load_configs(
  asset_server:Res<AssetServer>,
  mut loading:ResMut<AssetsLoading>,
  mut ai_data:ResMut<AiData>,
  query:Query<&mut AiRegister>,
){
  info!("Loading configs");
  for (i, AiRegister{name, mut index}) in query.iter().enumerate(){
    let path = format!("data/ai/{}.aiconfig.json",name);

    info!("Loading {:?}", path);
    let config:Handle<AiConfig> = asset_server.load(path);
    loading.0.push(config.clone().untyped());
    index = Some(i);
    ai_data.config_handles.push(config);
  }
}


fn parse_configs(
  mut ai_data:ResMut<AiData>,
  config_assets:Res<Assets<AiConfig>>,

){
  for handle in ai_data.config_handles.iter(){
    if let Some(config) = config_assets.get(handle.id()){

      for action in config.actions.iter(){
        info!(action.name);

      }

    }


  }

}


#[derive(Resource)]
struct AiConfigHandle();

#[derive(Resource, Default)]
pub struct AiData{
  config_handles:Vec<Handle<AiConfig>>
}   

#[derive(serde::Deserialize, Asset, TypePath)]
struct AiConfig {
  actions: Vec<Action>,
  behaviour:Vec<Behaviour>,
}

#[derive(serde::Deserialize)]
struct Action{
  name:String,
}

#[derive(serde::Deserialize)]
struct Behaviour{
  name:String,
}

#[derive(Component)]
pub struct Ai{
  next_think:Timer,
  target:Entity,
}