use std::str::FromStr;

use bevy::{prelude::*, text::cosmic_text::rustybuzz::script::NEWA, utils::HashMap};
use bevy_common_assets::json::JsonAssetPlugin;
use strum_macros::EnumString;

use crate::{asset_loader::AssetsLoading, scheduling::GameSchedule, state::GameState};
pub struct AiPlugin;

impl Plugin for AiPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(JsonAssetPlugin::<AiConfig>::new(&["aiconfig.json"]),)
      .init_resource::<AiDataCollection>()
      .add_systems(Startup, load_configs)
      .add_systems(OnExit(GameState::Loading), parse_configs)
      .add_systems(Update, do_action.in_set(GameSchedule::EntityUpdates));


    //app
    //.init_resource::<BehaviourCollection>();
  }
}

struct ActionHandle{
  data_ix:usize,
  action_ix:usize
}

struct BehaviourHandle{
  data_ix:usize,
  behaviour_ix:usize
}

#[derive(Component)]
pub struct Ai{
  action_timer:Timer,
  target:Option<Entity>,
}



fn do_action(mut query:Query<&mut Ai>, time:Res<Time>){
  for mut ai in query.iter_mut(){
    ai.action_timer.tick(time.delta());
    if ai.action_timer.just_finished(){


    }
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
  
  let Some(action_index) = action_indexes.get(&behaviour.action) else{
    panic!("Action not found: {:?}", behaviour.action);
  };
  
  let mut behaviour = Behaviour { 
    first_eval: behaviour.first_eval,
    action: *action_index,
    criteria: build_criteria(behaviour),
    options: build_options(behaviour, behaviour_indexes),
  };
  behaviour
}

fn build_options(
  behaviour_config: &BehaviourConfig,
  behaviour_indexes: &HashMap<String, usize>
) -> Vec<BehaviourOption> {
  behaviour_config.options.iter().map(|option_config| {
    let Some(behaviour_index) = behaviour_indexes.get(&option_config.behaviour) else{
      panic!("Behaviour not found: {:?}", option_config.behaviour);
    };
    BehaviourOption{
      behaviour_index: *behaviour_index,
      high:option_config.high.unwrap_or(f32::MAX),
      low:option_config.low.unwrap_or(f32::MIN),
    }
  }).collect::<Vec<BehaviourOption>>()

}

fn build_criteria(behaviour: &BehaviourConfig) -> Vec<EvaluationCriteria> {
  behaviour.criteria.iter().map(|criteria_config|{
    if let Ok(result) = Criteria::from_str(criteria_config.criteria.as_str()){
      EvaluationCriteria{ 
        criteria: result,
        offset:criteria_config.offset.unwrap_or(0.), 
        weight:criteria_config.weight.unwrap_or(1.),
      }
    }
    else{
      panic!("failed parsing criteria {:?}",criteria_config.criteria);
    }
  }).collect::<Vec<EvaluationCriteria>>()
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
        Action::Home { mut acceleration, mut max_speed } => { 
          acceleration = action_config.acceleration.unwrap_or(10.0);
          max_speed = action_config.max_speed.unwrap_or(20.0);
        },
        Action::Drift { mut variance, mut trend } =>{
          let variance_config = action_config.variance.unwrap_or((1.0,1.0));
          let trend_config = action_config.variance.unwrap_or((0.,0.));
          variance = Vec3::new(variance_config.0, 0., variance_config.1);
          trend = Vec3::new(trend_config.0, 0., trend_config.1);
        }
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
    max_speed:f32,
    acceleration:f32,
  },
  Drift{
    variance:Vec3,
    trend:Vec3,
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
  acceleration:Option<f32>,
  max_speed:Option<f32>,
  turn_time:Option<f32>,
  period:Option<f32>,
  offset:Option<f32>,
  variance:Option<(f32,f32)>,
  trend:Option<(f32,f32)>
}

#[derive(serde::Deserialize)]
struct BehaviourConfig{
  name:String,
  action:String,
  first_eval:f32,
  criteria:Vec<CriteriaConfig>,
  options:Vec<OptionConfig>,
}

#[derive(serde::Deserialize)]
pub struct CriteriaConfig{
  criteria:String,
  weight:Option<f32>,
  offset:Option<f32>,
}


#[derive(serde::Deserialize)]
struct OptionConfig{
  behaviour:String,
  high:Option<f32>,
  low:Option<f32>,
}


