use std::str::FromStr;

use bevy::{prelude::*, text::cosmic_text::rustybuzz::script::NEWA, utils::HashMap};
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
    ai_configs.0.push((name.clone(),config));
  }
  commands.insert_resource(ai_configs);
}


fn parse_configs(
  mut commands:Commands,
  ai_configs:Res<AiConfigCollection>,
  mut ai_data_collection:ResMut<AiDataCollection>,
  config_assets:Res<Assets<AiConfig>>,
){
  for (name, handle) in ai_configs.0.iter(){
    if let Some(config) = config_assets.get(handle.id()){
   
      //new ai data!
      let mut ai_data = AiData::default();
      ai_data.name = name.clone();

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
      for (index, behaviour) in config.behaviour.iter().enumerate(){
        ai_data.behaviours.push(build_behaviour(&ai_data.action_indexes, &ai_data.behaviour_indexes, behaviour));
        info!(behaviour.name);
      }

      //store our new data
      ai_data_collection.0.push(ai_data); 
    }
  }
  //dont need the configs anymore
  commands.remove_resource::<AiConfigCollection>();
}

fn build_behaviour(action_indexes: &HashMap<String, usize>, behaviour_indexes: &HashMap<String, usize>, behaviour: &BehaviourConfig) -> Behaviour {
  
  let Some(action_index) = behaviour_indexes.get(&behaviour.action) else{
    panic!("Behaviour not found: {:?}", behaviour.action);
  };
  
  let mut behaviour = Behaviour { 
    first_eval: behaviour.first_eval,
    action: *action_index,
    criteria: Vec::<EvaluationCriteria>::new(),
    options: Vec::<BehaviourOption>::new(),
  };

  behaviour
}



fn build_action(action_config: &ActionConfig) -> Action {

  if let Ok(result) = Action::from_str(action_config.action_name.as_str()){
    match (result) {
        Action::Idle => {},
        Action::SineWave { mut period, mut offset } =>  { 
          period = action_config.period.unwrap_or(20.0); 
          offset = action_config.offset.unwrap_or(0.0);
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


#[derive(EnumString, PartialEq)]
enum Criteria{
  Value,
  Random,
}

struct EvaluationCriteria{
  criteria:Criteria,
  weight:f32,
  offset:f32,
}


struct BehaviourOption{
  behaviour_index:usize,
  high:f32,
  low:f32,
}



struct Behaviour{
  first_eval:f32,
  action:usize,
  criteria:Vec<EvaluationCriteria>,
  options:Vec<BehaviourOption>,
}


#[derive(Resource, Default)]
pub struct AiDataCollection(Vec<AiData>);

#[derive(Resource, Default)]
pub struct AiData{
  name:String,
  behaviour_indexes:HashMap<String, usize>,
  action_indexes:HashMap<String, usize>,
  actions:Vec<Action>,
  behaviours:Vec<Behaviour>,
}   



#[derive(Resource, Default)]
pub struct AiConfigCollection(Vec<(String, Handle<AiConfig>)>);


#[derive(serde::Deserialize, Asset, TypePath)]
struct AiConfig {
  actions: Vec<ActionConfig>,
  behaviour:Vec<BehaviourConfig>,
}

#[derive(serde::Deserialize)]
struct ActionConfig{
  name:String,
  action_name:String,
  turn_time:Option<f32>,
  period:Option<f32>,
  offset:Option<f32>,
}

#[derive(serde::Deserialize)]
struct BehaviourConfig{
  name:String,
  action:String,
  first_eval:f32,
  criteria:Vec<CriteriaConfig>,
  choices:Vec<ChoiceConfig>,
}

#[derive(serde::Deserialize)]
pub struct CriteriaConfig{
  criteria:String,
  weight:Option<f32>,
  offset:Option<f32>,
}


#[derive(serde::Deserialize)]
struct ChoiceConfig{
  behaviour:String,
  high:Option<f32>,
  low:Option<f32>,
}


#[derive(Component)]
pub struct Ai{
  next_think:Timer,
  target:Entity,
}