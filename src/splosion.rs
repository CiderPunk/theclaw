use std::{array, f32::consts::PI};

use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};
use rand::Rng;

use crate::{movement::Velocity, scheduling::GameSchedule};

const SPLOSION_SHADER_PATH: &str = "shaders/animated_uv_shader.wgsl";

const SPLOSION_FRAMES: usize = 18;
const SPLOSION_ANIMATION_FPS: f32 = 15.0;
const SPLOSION_ANIMATION_LENGTH: f32 = (1. / SPLOSION_ANIMATION_FPS) * (SPLOSION_FRAMES - 2) as f32;

#[derive(Event)]
pub struct SplosionEvent {
  translation: Vec3,
  scale: f32,
  velocity: Vec3,
}

impl SplosionEvent {
  pub fn new(translation: Vec3, scale: f32, velocity: Vec3) -> Self {
    Self {
      translation,
      scale,
      velocity,
    }
  }
}

#[derive(Resource)]
struct SplosionQuad(Handle<Mesh>);

#[derive(Component)]
struct Splosion {
  timer: Timer,
}

#[derive(Resource)]
struct SplosionMaterialCollection {
  collection: [Handle<SplosionMaterial>; SPLOSION_FRAMES],
}

pub struct SplosionPlugin;
impl Plugin for SplosionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<SplosionMaterial>::default())
      .add_event::<SplosionEvent>()
      .add_systems(Startup, init_splosion)
      //.add_systems(Startup, _test.after(init_splosion))
      .add_systems(
        Update,
        (spawn_splosion, update_splosion).in_set(GameSchedule::EntityUpdates),
      );
  }
}

fn init_splosion(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<SplosionMaterial>>,
  asset_server: Res<AssetServer>,
) {
  let quad = meshes.add(Rectangle::new(8.0, 8.0));
  commands.insert_resource(SplosionQuad(quad));
  let splosion_texture: Handle<Image> = asset_server.load("sprites/splosion.png");

  //array of sequential numbers
  let frame_indexes: [usize; SPLOSION_FRAMES] = array::from_fn(|i| i + 1);

  //becomes array of sequential frame starting materials
  let frame_materials = frame_indexes.map(|i| {
    materials.add(SplosionMaterial {
      texture_atlas: Some(splosion_texture.clone()),
      alpha_mode: AlphaMode::Blend,
      settings: SplosionSettings {
        frame_offset: i as f32,
        frame_rate: SPLOSION_ANIMATION_FPS,
        ..default()
      },
    })
  });
  //store array as a resource
  commands.insert_resource(SplosionMaterialCollection {
    collection: frame_materials,
  });
}

fn _test(
  mut commands: Commands,
  mesh: Res<SplosionQuad>,
  materials: Res<SplosionMaterialCollection>,
) {
  for i in 0..SPLOSION_FRAMES {
    let material = materials.collection[i].clone();
    commands.spawn((
      Mesh3d(mesh.0.clone()),
      MeshMaterial3d(material),
      Transform::from_xyz(-5. * i as f32 + 40.0, 0., 0.)
        .with_rotation(Quat::from_rotation_x(PI * -0.5)),
    ));
  }
}

fn spawn_splosion(
  mut commands: Commands,
  mut ev_splosion_event: EventReader<SplosionEvent>,
  mesh: Res<SplosionQuad>,
  materials: Res<SplosionMaterialCollection>,
  time: Res<Time>,
) {
  for &SplosionEvent {
    translation,
    scale,
    velocity,
  } in ev_splosion_event.read()
  {
    let frame =
      ((time.elapsed_secs() * SPLOSION_ANIMATION_FPS) % SPLOSION_FRAMES as f32).floor() as usize;

    //info!("spawning frame {:?}", frame);
    let material = materials.collection[SPLOSION_FRAMES - frame - 1].clone();

    let mut rng = rand::thread_rng();
    let rotation = rng.gen_range(-1. ..1.);
    let mut transform = Transform::from_translation(translation)
      .with_scale(Vec3::new(scale, scale, scale))
      .with_rotation(Quat::from_rotation_x(PI * -0.5));
    transform.rotate_local_z(rotation * PI);
    commands.spawn((
      Splosion {
        timer: Timer::from_seconds(SPLOSION_ANIMATION_LENGTH, TimerMode::Once),
      },
      Mesh3d(mesh.0.clone()),
      MeshMaterial3d(material),
      transform,
      Velocity(velocity.clone()),
    ));
  }
}

fn update_splosion(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Splosion, &mut MeshMaterial3d<SplosionMaterial>)>,
  time: Res<Time>,
) {
  for (entity, mut splosion, _material) in &mut query {
    splosion.timer.tick(time.delta());
    if splosion.timer.just_finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

#[derive(Default, Clone, Copy, AsBindGroup, Debug, ShaderType)]
pub struct SplosionSettings {
  frame_offset: f32,
  frame_rate: f32,
  _webgl2_padding: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SplosionMaterial {
  #[uniform(0)]
  settings: SplosionSettings,
  #[texture(1)]
  #[sampler(2)]
  texture_atlas: Option<Handle<Image>>,

  alpha_mode: AlphaMode,
}

impl Material for SplosionMaterial {
  fn fragment_shader() -> ShaderRef {
    SPLOSION_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}
