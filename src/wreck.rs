use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{
  constants::GRAVITY,
  movement::{Acceleration, Roller, Velocity},
  scheduling::GameSchedule,
  splosion::SplosionEvent,
};

const WRECK_BLASTS: f32 = 4.0;

pub struct WreckPlugin;

impl Plugin for WreckPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_wrecks)
      .add_systems(Update, spawn_wrecks.in_set(GameSchedule::EntityUpdates))
      .add_systems(Update, update_wrecks.in_set(GameSchedule::DespawnEntities))
      //.add_observer(add_wreck_material)
      .add_event::<WreckedEvent>();
  }
}

#[derive(Event)]
pub struct WreckedEvent {
  scene: Handle<Scene>,
  translation: Vec3,
  quat: Quat,
  velocity: Vec3,
  roll_speed: f32,
  time_to_live: f32,
  blast_size: f32,
}

impl WreckedEvent {
  pub fn new(
    scene: Handle<Scene>,
    translation: Vec3,
    quat: Quat,
    velocity: Vec3,
    roll_speed: f32,
    time_to_live: f32,
    blast_size: f32,
  ) -> Self {
    Self {
      scene,
      translation,
      quat,
      velocity,
      roll_speed,
      time_to_live,
      blast_size,
    }
  }
}

#[derive(Resource)]
struct WreckMaterial(Handle<StandardMaterial>);

fn update_wrecks(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Wreck, &GlobalTransform, &Velocity)>,
  time: Res<Time>,
  mut ev_splosion_writer: EventWriter<SplosionEvent>,
) {
  for (entity, mut wreck, transform, velocity) in query.iter_mut() {
    wreck.time_to_blast.tick(time.delta());
    wreck.time_to_live.tick(time.delta());

    if wreck.time_to_blast.just_finished() {
      ev_splosion_writer.send(SplosionEvent::new(
        transform.translation() + Vec3::new(0., wreck.time_to_live.fraction_remaining() * 2.0, 0.0),
        wreck.blast_size * wreck.time_to_live.fraction(),
        velocity.0,
      ));
    }

    if wreck.time_to_live.just_finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn spawn_wrecks(
  mut commands: Commands,
  mut ev_wrecked_reader: EventReader<WreckedEvent>,
  mut ev_splosion_writer: EventWriter<SplosionEvent>,
) {
  for WreckedEvent {
    scene,
    translation,
    quat,
    velocity,
    roll_speed,
    time_to_live,
    blast_size,
  } in ev_wrecked_reader.read()
  {
    let mut observer = Observer::new(add_wreck_material);
    let entity = commands
      .spawn((
        SceneRoot(scene.clone()),
        Transform::from_translation(*translation).with_rotation(*quat),
        Velocity(*velocity),
        Wreck::new(*time_to_live, *blast_size),
        Roller {
          roll_speed: *roll_speed,
        },
        Acceleration::new(GRAVITY, 0.0, 40.),
      ))
      .id();
    observer.watch_entity(entity);
    commands.spawn(observer);

    ev_splosion_writer.send(SplosionEvent::new(
      *translation + Vec3::new(0., 0., -2.0),
      *blast_size,
      *velocity,
    ));
  }
}

fn add_wreck_material(
  trigger: Trigger<SceneInstanceReady>,
  mut commands: Commands,
  children: Query<&Children>,
  wreck_material: Res<WreckMaterial>,
) {
  //info!("adding material!");
  for descendant in children.iter_descendants(trigger.entity()) {
    //info!("descendant {:?}", descendant);
    commands
      .entity(descendant)
      .insert(MeshMaterial3d(wreck_material.0.clone()));
  }
}

fn init_wrecks(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let wreck_texture = asset_server.load("textures/wrecked_alpha.png");
  let wreck_material = materials.add(StandardMaterial {
    base_color: Color::Hsla(Hsla::WHITE),
    base_color_texture: Some(wreck_texture.clone()),
    emissive: LinearRgba::WHITE,
    emissive_texture: Some(wreck_texture.clone()),
    alpha_mode: AlphaMode::Mask(0.5),
    ..default()
  });
  commands.insert_resource(WreckMaterial(wreck_material));
}

#[derive(Component)]
pub struct Wreck {
  time_to_live: Timer,
  time_to_blast: Timer,
  blast_size: f32,
}

impl Wreck {
  pub fn new(time_to_live: f32, blast_size: f32) -> Self {
    Self {
      blast_size: blast_size,
      time_to_live: Timer::from_seconds(time_to_live, TimerMode::Once),
      time_to_blast: Timer::from_seconds(time_to_live / WRECK_BLASTS, TimerMode::Repeating),
    }
  }
}
