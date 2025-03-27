use std::str::FromStr;

use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::json::JsonAssetPlugin;
use strum_macros::EnumString;

use crate::{asset_loader::AssetsLoading, state::GameState};
pub struct AiPlugin;

impl Plugin for AiPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(JsonAssetPlugin::<AiConfig>::new(&["aiconfig.json"]),)
      .init_resource::<AiDataCollection>()
      .add_systems(Startup, load_configs)
      .add_systems(OnExit(GameState::Loading), parse_configs);
    //app
    //.init_resource::<BehaviourCollection>();
  }
}



#[derive(EnumString, PartialEq)]
enum Action{
  Idle,
  SineWave{
    period:f32,
    offset:f32,
  }, 
  Turn{
    time:f32,
  },
  Home{
    acceleration:f32,
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
  mut commands:Commands,
  asset_server:Res<AssetServer>,
  mut loading:ResMut<AssetsLoading>,
  query:Query<&mut AiRegister>,
){


  let mut ai_configs = AiConfigCollection::default();
  info!("Loading configs");
  for (i, AiRegister{name, mut index}) in query.iter().enumerate(){
    let path = format!("data/ai/{}.aiconfig.json",name);
    info!("Loading {:?}", path);
    let config:Handle<AiConfig> = asset_server.load(path);
    loading.0.push(config.clone().untyped());
    index = Some(i);
    ai_configs.0.push(config);
  }
  commands.insert_resource(ai_configs);
}


fn parse_configs(
  mut commands:Commands,
  asset_server:Res<AssetServer>,
  ai_configs:Res<AiConfigCollection>,
  mut ai_data_collection:ResMut<AiDataCollection>,
  config_assets:Res<Assets<AiConfig>>,
){
  for handle in ai_configs.0.iter(){
    if let Some(config) = config_assets.get(handle.id()){
   
      //new ai data!
      let mut ai_data = AiData::default();
      
      //get path as the name maybe...
      if let Some(path) = asset_server.get_path(handle.id()){
        ai_data.name = path.to_string();
      }
      //build hashmap of indexes
      for (index, action_config) in config.actions.iter().enumerate(){
        ai_data.action_indexes.insert(action_config.name.clone(), index);
        ai_data.actions.push(build_action(action_config));
        info!(action_config.name);
      }
      for (index, behaviour) in config.behaviour.iter().enumerate(){
        ai_data.action_indexes.insert(behaviour.name.clone(), index);
        info!(behaviour.name);
      }

      //store our new data
      ai_data_collection.0.push(ai_data); 
    }
  }
  //dont need the configs anymore
  commands.remove_resource::<AiConfigCollection>();
}

fn build_action(action_config: &ActionConfig) -> Action {

  if let Ok(result) = Action::from_str(action_config.action_name.as_str()){
    match (result) {
        Action::Idle => {},
        Action::SineWave { mut period, mut offset } =>  { 
          period = action_config.period.unwrap_or(20.0); 
        },
        Action::Turn { mut time } => { 
          time = action_config.turn_time.unwrap_or(1.2); 
        },
        Action::Home { mut acceleration } => {},
    }
    result
  }
  else{
    panic!("failed parsing action {:?}",action_config.action_name);
  }
}


#[derive(Resource, Default)]
pub struct AiDataCollection(Vec<AiData>);

#[derive(Resource, Default)]
pub struct AiData{
  name:String,
  behaviour_indexes:HashMap<String, usize>,
  action_indexes:HashMap<String, usize>,
  actions:Vec<Action>,
}   



#[derive(Resource, Default)]
pub struct AiConfigCollection(Vec<Handle<AiConfig>>);


#[derive(serde::Deserialize, Asset, TypePath)]
struct AiConfig {
  name: String,
  actions: Vec<ActionConfig>,
  behaviour:Vec<BehaviourConfig>,
}

#[derive(serde::Deserialize)]
struct ActionConfig{
  name:String,
  action_name:String,
  turn_time:Option<f32>,
  period:Option<f32>,
}

#[derive(serde::Deserialize)]
struct BehaviourConfig{
  name:String,
}

#[derive(Component)]
pub struct Ai{
  next_think:Timer,
  target:Entity,
}