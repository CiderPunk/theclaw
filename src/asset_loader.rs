use bevy::prelude::*;

use crate::state::{GameState, GameStateEvent};


const BULLET_COLOUR:Color = Color::srgb(2.0, 1.8, 0.2);
const BULLET_SIZE:f32 = 0.5;


#[derive(Resource, Default)]
pub struct SceneAssets{
  pub ship: Handle<Scene>,
  pub sidewinder: Handle<Scene>,
  pub hook: Handle<Scene>,
  pub bullet: Handle<Mesh>,
  pub bullet_material:Handle<StandardMaterial>,
  pub font:Handle<Font>,
}

#[derive(Resource)]
struct ShipScene(Handle<Gltf>);

#[derive(Resource)]
struct GameFont(Handle<Font>);


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

  let font = asset_server.load("fonts/OpenSans_Condensed-Bold.ttf");
  commands.insert_resource(GameFont(font));
}

fn extract_assets(
  mut scene_assets:ResMut<SceneAssets>,
  ship_scene: Res<ShipScene>,
  gltf_assets: Res<Assets<Gltf>>,
  mut ev_game_state_writer: EventWriter<GameStateEvent>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  game_font:Res<GameFont>,
){
  let Some(gltf) = gltf_assets.get(&ship_scene.0) else{
    return;
  };

  info!("extracting assets");

  *scene_assets = SceneAssets{
    ship: gltf.named_scenes["ClawShip"].clone(),
    sidewinder: gltf.named_scenes["Sidewinder"].clone(),
    hook: gltf.named_scenes["Claw"].clone(),
    bullet: meshes.add( Sphere::new(BULLET_SIZE).mesh().kind(bevy::render::mesh::SphereKind::Ico { subdivisions: 3 })),
    bullet_material: materials.add(BULLET_COLOUR),
    font: game_font.0.clone(),
  };
  //signal ready for game start
  ev_game_state_writer.send(GameStateEvent::new(GameState::Playing));

}