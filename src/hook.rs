use bevy::prelude::*;

use crate::{
  collision_detection::{CollisionEvent, Player},
  movement::{Acceleration, Velocity},
  scheduling::GameSchedule,
};

const HOOK_RETURN_DISTANCE: f32 = 40.0;
const HOOK_RECLAIM_DISTANCE: f32 = 3.5;
pub const HOOK_LAUNCH_SPEED: f32 = 60.0;
pub const HOOK_MAX_SPEED: f32 = 80.0;
pub const HOOK_RETURN_ACCELERATION: f32 = 800.0;
pub const HOOK_DAMPING: f32 = 5.0;

pub struct HookPlugin;
impl Plugin for HookPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_hook.in_set(GameSchedule::UserInput))
      .add_systems(Update, retrieve_hook.in_set(GameSchedule::DespawnEntities))
      .add_event::<HookReturnedEvent>();
  }
}

#[derive(Component)]
#[require(Acceleration, Velocity, Player)]
pub struct Hook {
  owner: Entity,
  returning: bool,
  target: Option<Entity>,
}
impl Hook {
  pub fn new(owner: Entity) -> Self {
    Self {
      owner: owner,
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

fn update_hook(
  mut query: Query<(&mut Hook, &GlobalTransform, &mut Acceleration)>,
  owner_query: Query<&GlobalTransform>,
  mut ev_hook_returned: EventWriter<HookReturnedEvent>,
) {
  let Ok((mut hook, hook_transform, mut acceleration)) = query.get_single_mut() else {
    return;
  };

  let Ok(owner_transform) = owner_query.get(hook.owner) else {
    return;
  };

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
    info!("Hook returning");
  }
}

fn retrieve_hook(
  mut commands: Commands,
  mut ev_hook_returned: EventReader<HookReturnedEvent>,
  query: Query<Entity, With<Hook>>,
) {
  for &HookReturnedEvent { target } in ev_hook_returned.read() {
    let Ok(entity) = query.get_single() else {
      return;
    };
    commands.entity(entity).despawn();
  }
}

fn apply_collisions(mut ev_collision: EventReader<CollisionEvent>) {
  for &CollisionEvent { entity, collided } in ev_collision.read() {}
}
