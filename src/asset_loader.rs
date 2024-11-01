use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SceneAssets{
  pub ship: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
  fn build(&self, app: &mut App){
    app.init_resource::<SceneAssets>()
      .add_systems(Startup, load_assets);
  }
}

fn load_assets(mut scene_assets:ResMut<SceneAssets>, asset_server: Res<AssetServer>){
  *scene_assets = SceneAssets{
    ship: asset_server.load("models/ship.glb#Scene0"),
  }
}