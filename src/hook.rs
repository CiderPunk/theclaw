use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
  collision_detection::{Collider, CollisionEvent, Player},
  movement::{Acceleration, Roller, Velocity},
  scheduling::GameSchedule,
};

const HOOK_RETURN_DISTANCE: f32 = 40.0;
const HOOK_RECLAIM_DISTANCE: f32 = 5.0;
pub const HOOK_LAUNCH_SPEED: f32 = 60.0;
pub const HOOK_MAX_SPEED: f32 = 80.0;
pub const HOOK_RETURN_ACCELERATION: f32 = 800.0;
pub const HOOK_DAMPING: f32 = 5.0;
pub const HOOK_COLLISION_RADIUS: f32 = 1.0;
pub const HOOK_CENTERING_SPEED: f32 = 3.0;

pub struct HookPlugin;
impl Plugin for HookPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_hook.in_set(GameSchedule::UserInput))
      .add_systems(Update, retrieve_hook.in_set(GameSchedule::DespawnEntities))
      .add_systems(
        Update,
        (apply_collisions, center_hooked).in_set(GameSchedule::EntityUpdates),
      )
      .add_event::<HookReturnedEvent>();
  }
}

#[derive(Component, Default)]
pub struct Hookable {
  pub translation: Vec3,
  pub rotation: Quat,
}

impl Hookable {
  pub fn new(translation: Vec3, rotation: Quat) -> Self {
    Self {
      translation,
      rotation,
    }
  }
}

#[derive(Component, Default)]
pub struct Hooked {
  time: Stopwatch,
  initial_position: Vec3,
  initial_rotation: Quat,
}

#[derive(Component)]
#[require(Acceleration, Velocity, Player)]
pub struct Hook {
  owner: Entity,
  pub returning: bool,
  target: Option<Entity>,
}
impl Hook {
  pub fn new(owner: Entity) -> Self {
    Self {
      owner,
      returning: false,
      target: None,
    }
  }
}

#[derive(Event)]
pub struct HookReturnedEvent {
  pub target: Option<Entity>,
}

impl HookReturnedEvent {
  pub fn new(target: Option<Entity>) -> Self {
    Self { target }
  }
}

pub fn hook_builder(
  owner: Entity,
  start: Vec3,
  launcher_veloctiy: Vec3,
  scene: Handle<Scene>,
) -> (
  Hook,
  Player,
  SceneRoot,
  Velocity,
  Acceleration,
  Transform,
  Collider,
) {
  (
    Hook::new(owner),
    Player,
    SceneRoot(scene.clone()),
    Velocity(launcher_veloctiy + Vec3::new(-HOOK_LAUNCH_SPEED, 0., 0.)),
    Acceleration {
      acceleration: Vec3::ZERO,
      damping: HOOK_DAMPING,
      max_speed: HOOK_MAX_SPEED,
    },
    Transform::from_translation(start),
    Collider::new(HOOK_COLLISION_RADIUS, 0.0),
  )
}

fn update_hook(
  mut query: Query<(
    &mut Hook,
    &GlobalTransform,
    &mut Transform,
    &mut Acceleration,
  )>,
  owner_query: Query<&GlobalTransform>,
  mut ev_hook_returned: EventWriter<HookReturnedEvent>,
) {
  let Ok((mut hook, hook_transform, mut transform, mut acceleration)) = query.get_single_mut()
  else {
    return;
  };

  let Ok(owner_transform) = owner_query.get(hook.owner) else {
    return;
  };

  transform.look_at(owner_transform.translation(), Vec3::Y);
  transform.rotate_local_y(PI * 0.5);

  let diff = hook_transform.translation() - owner_transform.translation();
  let diff_squared = diff.length_squared();
  if hook.returning {
    let acc = diff.normalize() * -HOOK_RETURN_ACCELERATION;
    acceleration.acceleration = acc;
    if diff_squared < HOOK_RECLAIM_DISTANCE * HOOK_RECLAIM_DISTANCE {
      ev_hook_returned.send(HookReturnedEvent::new(hook.target));
    }
  } else if diff_squared > HOOK_RETURN_DISTANCE * HOOK_RETURN_DISTANCE {
    hook.returning = true;
    //info!("Hook returning");
  }
}

fn retrieve_hook(
  mut commands: Commands,
  mut ev_hook_returned: EventReader<HookReturnedEvent>,
  query: Query<Entity, With<Hook>>,
) {
  for &HookReturnedEvent { target } in ev_hook_returned.read() {
    //despawn our hook
    let Ok(entity) = query.get_single() else {
      return;
    };

    info!("hook returned, captive: {:?}", target);
    if let Some(target) = target {
      commands.entity(entity).remove_children(&[target]);
    }
    commands.entity(entity).despawn_recursive();
  }
}

fn apply_collisions(
  mut commands: Commands,
  mut ev_collision: EventReader<CollisionEvent>,
  mut hook_query: Query<(&mut Hook, &GlobalTransform)>,
  mut target_query: Query<
    (&mut Transform, &mut Velocity, &GlobalTransform),
    (With<Hookable>, Without<Hook>),
  >,
) {
  for &CollisionEvent {
    player: entity,
    other: collided,
  } in ev_collision.read()
  {
    let Ok((mut hook, hook_transform)) = hook_query.get_mut(entity) else {
      continue;
    };
    let Ok((mut transform, mut target_velocity, target_transform)) = target_query.get_mut(collided)
    else {
      continue;
    };
    target_velocity.0 = Vec3::ZERO;
    //target_acceleration.acceleration = Vec3::ZERO;
    hook.returning = true;
    hook.target = Some(collided);
    commands
      .entity(entity)
      .remove::<Collider>()
      .add_child(collided);
    transform.translation = target_transform.translation() - hook_transform.translation();

    commands.entity(collided).remove::<Roller>().insert(Hooked {
      time: Stopwatch::new(),
      initial_position: transform.translation,
      initial_rotation: transform.rotation,
    });
  }
}

fn center_hooked(mut query: Query<(&mut Hooked, &mut Transform, &Hookable)>, time: Res<Time>) {
  let Ok((mut hooked, mut transform, hookable)) = query.get_single_mut() else {
    return;
  };
  hooked.time.tick(time.delta());
  let ratio = (hooked.time.elapsed_secs() * HOOK_CENTERING_SPEED).clamp(0.0, 1.0);

  transform.translation = hooked.initial_position.lerp(hookable.translation, ratio);
  transform.rotation = hooked.initial_rotation.lerp(hookable.rotation, ratio);
}
