use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
struct AiPlugin;

impl Plugin for AiPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(JsonAssetPlugin::<AiConfig>::new(&["aiconfig.json"]),)
      .add_systems(Startup, load_configs);
    //app
    //.init_resource::<BehaviourCollection>();
  }
}


fn load_configs(asset_server:Res<AssetServer>){

  let ai_config = AiConfigHandle(asset_server.load("trees.level.json"));
}



#[derive(Resource)]
struct AiConfigHandle(Handle<AiConfig>);


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