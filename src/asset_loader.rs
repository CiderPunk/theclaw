use bevy::prelude::*;

use crate::ordering::GameLoading;

#[derive(Resource, Default)]
pub struct SceneAssets{
  pub ship: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
  fn build(&self, app: &mut App){
    app.init_resource::<SceneAssets>()
      .add_systems(PreStartup, load_assets.in_set(GameLoading));
  }
}

fn load_assets(mut scene_assets:ResMut<SceneAssets>, asset_server: Res<AssetServer>){
  *scene_assets = SceneAssets{
    ship: asset_server.load("models/ship2.glb#Scene0"),
  }
}
