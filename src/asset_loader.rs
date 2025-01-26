use bevy::prelude::*;

use crate::state::{GameState, GameStateEvent};


#[derive(Resource, Default)]
pub struct SceneAssets{
  pub ship: Handle<Scene>,
  pub sidewinder: Handle<Scene>,
}

#[derive(Resource)]
struct ShipScene(Handle<Gltf>);


pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
  fn build(&self, app: &mut App){
    app.init_resource::<SceneAssets>()
    .add_systems(Startup, load_assets.run_if(in_state(GameState::Loading)))
    .add_systems(Update, extract_assets.run_if(in_state(GameState::Loading))); 
  }
}

fn load_assets(mut commands:Commands, asset_server: Res<AssetServer>){
  info!("loading assets");
  let gltf = asset_server.load("models/ship2.glb");
  commands.insert_resource(ShipScene(gltf));
}

fn extract_assets(
  mut scene_assets:ResMut<SceneAssets>,
  ship_scene: Res<ShipScene>,
  gltf_assets: Res<Assets<Gltf>>,
  mut ev_game_state_writer: EventWriter<GameStateEvent>
){
  let Some(gltf) = gltf_assets.get(&ship_scene.0) else{
    return;
  };
  info!("extracting assets");


  *scene_assets = SceneAssets{
    ship: gltf.named_scenes["ClawShip"].clone(),
    sidewinder: gltf.named_scenes["Sidewinder"].clone(),
  };
  //signal ready for game start
  ev_game_state_writer.send(GameStateEvent::new(GameState::Playing));

}