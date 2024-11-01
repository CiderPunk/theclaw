use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0, 0.0);

#[derive(Component)]
pub struct Ship;
pub struct ShipPlugin;

impl Plugin for ShipPlugin{
  fn build(&self, app: &mut App){
    app.add_systems(Startup, spawn_ship);
  }
}


fn spawn_ship(mut commands:Commands, scene_assets:Res<SceneAssets>){
  commands.spawn((
    SceneBundle{ 
      scene:scene_assets.ship.clone(),
      transform: Transform::from_translation(STARTING_TRANSLATION),
      ..default()
    },
    Ship,
  ));

}

