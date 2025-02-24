use std::f32::consts::PI;

use bevy::{prelude::*, render::{extract_component::ComponentUniforms, render_resource::{AsBindGroup, ShaderRef}}};
use rand::Rng;

use crate::{movement::Velocity, scheduling::GameSchedule};

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

#[derive(Component)]
struct Splosion{
  timer:Timer,
}



pub struct SplosionPlugin;
impl Plugin for SplosionPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<SplosionMaterial>::default())
      .add_event::<SplosionEvent>()
      .add_systems(Startup, init_splosion)
      .add_systems(Update, (spawn_splosion, update_splosion).in_set(GameSchedule::EntityUpdates));

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
  commands.insert_resource(SplosionMaterialResource(material_handle));
}

fn spawn_splosion(mut commands:Commands, mut ev_splosion_event: EventReader<SplosionEvent>, mesh:Res<SplosionQuad>, material:Res<SplosionMaterialResource> ){
  for &SplosionEvent{ translation, scale, velocity } in ev_splosion_event.read(){
    info!("spawning");
    let mat = material.0.clone();

    let mut rng = rand::thread_rng();
    let rotation = rng.gen_range(-1. ..1.);


    
    let mut transform = Transform::from_translation(translation).with_scale(Vec3::new(scale, scale, scale)).with_rotation(Quat::from_rotation_x(PI * -0.5));
    transform.rotate_local_z(rotation * PI);
    commands.spawn((
      Splosion{ timer:Timer::from_seconds(2.0, TimerMode::Once) },
      Mesh3d(mesh.0.clone()),
      MeshMaterial3d(mat),
      transform,
      Velocity(velocity.clone()),
    ));
  }
}


fn update_splosion(mut commands:Commands, mut query:Query<(Entity, &mut Splosion, &mut MeshMaterial3d<SplosionMaterial>)>, time:Res<Time>){
  for (entity, mut splosion, mut material) in &mut query {
    splosion.timer.tick(time.delta());
    if splosion.timer.just_finished(){
      commands.entity(entity).despawn_recursive();
    }
  }

}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SplosionMaterial{
  #[uniform(0)]
  pub frame: f32,
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



