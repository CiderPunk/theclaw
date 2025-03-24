use std::{array, f32::consts::PI};

use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};
use rand::Rng;

use crate::{asset_loader::AssetsLoading, movement::Velocity, scheduling::GameSchedule};

const EFFECT_SPRITE_SHADER_PATH: &str = "shaders/animated_uv_shader_v2.wgsl";


const SPLOSION_FRAMES: usize = 17;
const SPLOSION_DISPLAY_FRAMES: f32 = 16.;
const SPLOSION_HORIZONTAL_FRAMES:f32 = 4.;
const SPLOSION_VERTICAL_FRAMES:f32 = 4.;
const SPLOSION_ANIMATION_FPS: f32 = 15.0;
const SPLOSION_ANIMATION_LENGTH: f32 = (1. / SPLOSION_ANIMATION_FPS) * (SPLOSION_FRAMES - 1) as f32;


const RICOCHET_FRAMES: usize = 8;
const RICOCHET_DISPLAY_FRAMES: f32 = 7.;
const RICOCHET_HORIZONTAL_FRAMES:f32 = 4.;
const RICOCHET_VERTICAL_FRAMES:f32 = 2.;
const RICOCHET_ANIMATION_FPS: f32 = 15.0;
const RICOCHET_ANIMATION_LENGTH: f32 = (1. / RICOCHET_ANIMATION_FPS) * (RICOCHET_FRAMES - 1) as f32;


#[derive(Clone, Copy)]
pub enum EffectSpriteType{
  Splosion,
  Ricochet,
}

#[derive(Event)]
pub struct EffectSpriteEvent {
  translation: Vec3,
  scale: f32,
  velocity: Vec3,
  effect:EffectSpriteType,
}

impl EffectSpriteEvent {
  pub fn new(translation: Vec3, scale: f32, velocity: Vec3, effect: EffectSpriteType) -> Self {
    Self {
      translation,
      scale,
      velocity,
      effect,
    }
  }
}

#[derive(Resource)]
struct EffectQuad(Handle<Mesh>);

#[derive(Component)]
struct EffectSprite {
  timer: Timer,
}

#[derive(Resource)]
struct EffectMaterialCollection {
  splosion: [Handle<EffectSpriteMaterial>; SPLOSION_FRAMES],
  ricochet: [Handle<EffectSpriteMaterial>; RICOCHET_FRAMES]
}

pub struct EffectSpritePlugin;
impl Plugin for EffectSpritePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<EffectSpriteMaterial>::default())
      .add_event::<EffectSpriteEvent>()
      .add_systems(Startup, init_effect_sprites)
      //.add_systems(Startup, _test.after(init_splosion))
      .add_systems(
        Update,
        (spawn_effect_sprites, update_effect_sprites).in_set(GameSchedule::EntityUpdates),
      );
  }
}

fn init_effect_sprites(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<EffectSpriteMaterial>>,
  asset_server: Res<AssetServer>,
  mut loading:ResMut<AssetsLoading>,
) {
  let quad = meshes.add(Rectangle::new(8.0, 8.0));
  commands.insert_resource(EffectQuad(quad));
  let splosion_texture: Handle<Image> = asset_server.load("sprites/splosion.png");
  let ricochet_texture: Handle<Image> = asset_server.load("sprites/ricochet.png");
    
  loading.0.push(splosion_texture.clone().untyped());
  loading.0.push(ricochet_texture.clone().untyped());

  let splosion_material_collection = create_material_collection::<SPLOSION_FRAMES>(
    &mut materials,
    splosion_texture,
    SPLOSION_DISPLAY_FRAMES,
    SPLOSION_HORIZONTAL_FRAMES, 
    SPLOSION_VERTICAL_FRAMES,
    SPLOSION_ANIMATION_FPS
  );
  let ricochet_material_collection = create_material_collection::<RICOCHET_FRAMES>(
    &mut materials,
    ricochet_texture, 
    RICOCHET_DISPLAY_FRAMES,
    RICOCHET_HORIZONTAL_FRAMES, 
    RICOCHET_VERTICAL_FRAMES,
    RICOCHET_ANIMATION_FPS
  );
  //store array as a resource
  commands.insert_resource(EffectMaterialCollection {
    splosion: splosion_material_collection,
    ricochet: ricochet_material_collection,
  });
}

fn create_material_collection<const FRAME_COUNT:usize>(
  materials: &mut ResMut<Assets<EffectSpriteMaterial>>,
  texture:Handle<Image>,
  display_frames:f32,
  horizontal_frames:f32,
  vertical_frames:f32,
  fps:f32,
) -> [Handle<EffectSpriteMaterial>; FRAME_COUNT]{
  //array of sequential numbers
  let indexes: [usize; FRAME_COUNT] = array::from_fn(|i| i + 1);
  //becomes array of sequential frame starting materials
  indexes.map(|i| {
    materials.add(EffectSpriteMaterial {
      texture_atlas: Some(texture.clone()),
      alpha_mode: AlphaMode::Blend,
      settings: EffectSpriteSettings {
        frame_offset: i as f32,
        frame_rate: fps,
        frame_count:FRAME_COUNT as f32,
        frames_deep:vertical_frames,
        frames_wide:horizontal_frames,
        display_frames:display_frames,
        ..default()
      },
    })
  })
}



fn get_material(effect:&EffectSpriteType, effect_materials: &Res<EffectMaterialCollection>, time: &Res<Time>) ->Handle<EffectSpriteMaterial>{
  match effect {
    EffectSpriteType::Splosion=>{
      let frame =((time.elapsed_secs() * SPLOSION_ANIMATION_FPS) % SPLOSION_FRAMES as f32).floor() as usize;
      effect_materials.splosion[SPLOSION_FRAMES - frame - 1].clone()
    },
    EffectSpriteType::Ricochet=>{
      let frame =((time.elapsed_secs() * RICOCHET_ANIMATION_FPS) % RICOCHET_FRAMES as f32).floor() as usize;
      effect_materials.ricochet[RICOCHET_FRAMES - frame - 1].clone()
    },  
  }
}

fn get_animation_length(effect:EffectSpriteType) ->f32{
  match effect {
    EffectSpriteType::Splosion=>SPLOSION_ANIMATION_LENGTH,
    EffectSpriteType::Ricochet=>RICOCHET_ANIMATION_LENGTH,  
  }
}


fn spawn_effect_sprites(
  mut commands: Commands,
  mut ev_effect_reader: EventReader<EffectSpriteEvent>,
  mesh: Res<EffectQuad>,
  effect_materials: Res<EffectMaterialCollection>,
  time: Res<Time>,
) {
  for &EffectSpriteEvent {
    translation,
    scale,
    velocity,
    effect,
  } in ev_effect_reader.read()
  {
    let mut rng = rand::thread_rng();
    let rotation = rng.gen_range(-1. ..1.);
    let mut transform = Transform::from_translation(translation)
      .with_scale(Vec3::new(scale, scale, scale))
      .with_rotation(Quat::from_rotation_x(PI * -0.5));
    transform.rotate_local_z(rotation * PI);

    commands.spawn((
      EffectSprite {
        timer: Timer::from_seconds(get_animation_length(effect), TimerMode::Once),
      },
      Mesh3d(mesh.0.clone()),
      MeshMaterial3d(get_material(&effect, &effect_materials, &time)),
      transform,
      Velocity(velocity),
    ));
  }
}

fn update_effect_sprites(
  mut commands: Commands,
  mut query: Query<(Entity, &mut EffectSprite)>,
  time: Res<Time>,
) {
  for (entity, mut sprite) in &mut query {
    sprite.timer.tick(time.delta());
    if sprite.timer.just_finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

#[derive(Default, Clone, Copy, AsBindGroup, Debug, ShaderType)]
pub struct EffectSpriteSettings {
  frame_offset: f32,
  frame_rate: f32,
  frames_wide:f32, 
  frames_deep:f32,
  frame_count:f32,
  display_frames:f32,
  _webgl2_padding: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EffectSpriteMaterial {
  #[uniform(0)]
  settings: EffectSpriteSettings,
  #[texture(1)]
  #[sampler(2)]
  texture_atlas: Option<Handle<Image>>,
  alpha_mode: AlphaMode,
}

impl Material for EffectSpriteMaterial {
  fn fragment_shader() -> ShaderRef {
    EFFECT_SPRITE_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}
