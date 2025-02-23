use std::f32::consts::PI;

use bevy::{pbr::{MaterialPipeline, MaterialPipelineKey}, prelude::*, render::{mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef}, render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat}}};

use crate::{asset_loader::SceneAssets, camera::CAMERA_LOCATION, movement::Velocity, scheduling::GameSchedule};

const SPLOSION_SHADER_PATH: &str = "shaders/animated_uv_shader.wgsl";
//const ATTRIBUTE_FRAME: MeshVertexAttribute =
//    MeshVertexAttribute::new("Frame", 988540917, VertexFormat::Float32x4);

    
#[derive(Event)]
pub struct SplosionEvent{
  translation:Vec3,
  scale:f32,
  velocity:Vec3,
}

impl SplosionEvent{
  pub fn new(translation:Vec3, scale:f32, velocity:Vec3) -> Self{
    Self{ translation, scale, velocity }
  }
}

#[derive(Resource)]
struct SplosionQuad(Handle<Mesh>);

#[derive(Resource)]
struct SplosionMaterialResource(Handle<SplosionMaterial>);


pub struct SplosionPlugin;
impl Plugin for SplosionPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<SplosionMaterial>::default())
      .add_event::<SplosionEvent>()
      .add_systems(Startup, init_splosion)
      .add_systems(Update, (spawn_splosion).in_set(GameSchedule::EntityUpdates));

  } 
}

fn init_splosion(mut commands:Commands, mut meshes:ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<SplosionMaterial>>,  asset_server: Res<AssetServer>){
  let quad = meshes.add(Rectangle::new(8.0, 8.0));
  commands.insert_resource(SplosionQuad(quad));

  let splosion_texture:Handle<Image> = asset_server.load("sprites/splosion.png");
  let material_handle = materials.add(SplosionMaterial {
    frame: 0.,
    texture_atlas: Some(splosion_texture),
    alpha_mode: AlphaMode::Blend,
  });


  /*
  let material_handle = materials.add(StandardMaterial {
    base_color_texture: Some(splosion_texture.clone()),
    alpha_mode: AlphaMode::Blend,
    unlit: true,
    ..default()
  });
 */
  commands.insert_resource(SplosionMaterialResource(material_handle));
}

fn spawn_splosion(mut commands:Commands, mut ev_splosion_event: EventReader<SplosionEvent>, mesh:Res<SplosionQuad>, material:Res<SplosionMaterialResource> ){
  for &SplosionEvent{ translation, scale, velocity } in ev_splosion_event.read(){
    info!("spawning");
    commands.spawn((
      Mesh3d(mesh.0.clone()),
      MeshMaterial3d(material.0.clone()),
      Transform::from_translation(translation).with_rotation(Quat::from_rotation_x(PI * -0.5)).with_scale(Vec3::new(scale, scale, scale)),
      Velocity(velocity.clone()),
    ));
  }
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SplosionMaterial{
  #[uniform(0)]
  frame: f32,
  #[texture(1)]
  #[sampler(2)]
  texture_atlas: Option<Handle<Image>>,
  alpha_mode: AlphaMode,
}

impl Material for SplosionMaterial{
  fn fragment_shader() -> ShaderRef {
    SPLOSION_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
}
}



