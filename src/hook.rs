use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
  asset_loader::SceneAssets,
  collision_detection::{Collider, CollisionEvent, Player},
  movement::{Acceleration, Velocity},
  scheduling::GameSchedule,
};

const HOOK_RETURN_DISTANCE: f32 = 40.0;
const HOOK_RECLAIM_DISTANCE: f32 = 5.0;
pub const HOOK_LAUNCH_SPEED: f32 = 60.0;
pub const HOOK_MAX_SPEED: f32 = 80.0;
pub const HOOK_RETURN_ACCELERATION: f32 = 800.0;
pub const HOOK_DAMPING: f32 = 5.0;
pub const HOOK_COLLISION_RADIUS: f32 = 2.0;
pub const HOOK_CENTERING_SPEED:f32 = 10.0;

pub struct HookPlugin;
impl Plugin for HookPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_hook.in_set(GameSchedule::UserInput))
      .add_systems(Update, retrieve_hook.in_set(GameSchedule::DespawnEntities))
      .add_systems(
        Update,
        (hook_launch, apply_collisions, update_hook_angle, center_hooked).in_set(GameSchedule::EntityUpdates),
      )
      .add_event::<HookReturnedEvent>()
      .add_event::<HookLaunchEvent>();
  }
}

#[derive(Component, Default)]
pub struct Hookable{
  translation:Vec3,
  rotation:Quat,
}

impl Hookable{
  pub fn new(translation:Vec3, rotation:Quat)->Self{
    Self{translation, rotation}
  }
}


#[derive(Component, Default)]
pub struct Hooked {
  time: Stopwatch,
}
#[derive(Component)]
pub struct Captured {
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

#[derive(Event)]
pub struct HookLaunchEvent {
  owner: Entity,
  location: Vec3,
  base_velocity: Vec3,
}

impl HookLaunchEvent {
  pub fn new(owner: Entity, location: Vec3, base_velocity: Vec3) -> Self {
    Self {
      owner,
      location,
      base_velocity,
    }
  }
}

fn hook_launch(
  mut commands: Commands,
  mut ev_hook_launch: EventReader<HookLaunchEvent>,
  scene_assets: Res<SceneAssets>,
) {
  for &HookLaunchEvent {
    owner,
    location,
    base_velocity,
  } in ev_hook_launch.read()
  {
    commands.spawn((
      Hook::new(owner),
      Player,
      SceneRoot(scene_assets.hook.clone()),
      Velocity(base_velocity + Vec3::new(-HOOK_LAUNCH_SPEED, 0., 0.)),
      Acceleration {
        acceleration: Vec3::ZERO,
        damping: HOOK_DAMPING,
        max_speed: HOOK_MAX_SPEED,
      },
      Transform::from_translation(location),
      Collider::new(HOOK_COLLISION_RADIUS),
    ));
  }
}

fn update_hook_angle(mut query: Query<(&mut Transform, &Velocity), With<Hook>>) {
  let Ok((mut transform, velocity)) = query.get_single_mut() else {
    return;
  };

  //let angle = (velocity.z / velocity.x).atan() + (PI * 0.5); 
  //transform.rotation = Quat::from_rotation_y(angle);
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

    //despawn our hook
    let Ok(entity) = query.get_single() else {
      return;
    };
    commands.entity(entity).despawn_recursive();
  }
}


fn apply_collisions(
  mut commands: Commands,
  mut ev_collision: EventReader<CollisionEvent>,
  mut hook_query: Query<(&mut Hook, &GlobalTransform)>,
  mut target_query: Query<(&mut Transform, &mut Velocity, &GlobalTransform, &Hookable), Without<Hook>>,
) {
  for &CollisionEvent { entity, collided } in ev_collision.read() {
    let Ok((mut hook, hook_transform)) = hook_query.get_mut(entity) else {
      continue;
    };
    let Ok((mut transform, mut velocity, target_transform, hookable)) = target_query.get_mut(collided) else{ 
      continue;
    };
    hook.returning = true;
    commands.entity(entity).add_child(collided);
    transform.translation = target_transform.translation() - hook_transform.translation();
    velocity.0 = -transform.translation.normalize() * HOOK_CENTERING_SPEED;
    //target_transform.translation = Vec3::ZERO;
    commands.entity(entity).remove::<Collider>();
    commands.entity(collided).insert(Hooked{ time:Stopwatch::new() });

  }
}

fn center_hooked(mut query:Query<(&mut Hooked,&mut Transform, &mut Velocity)>, time:Res<Time>){
  let Ok((mut hooked, mut transform, mut velocity)) = query.get_single_mut() else {
    return;
  };
  hooked.time.tick(time.delta());
  //transform.rotation = Quat::lerp(self, end, s)
  if transform.translation.length_squared() < 1.0{
    velocity.0 = Vec3::ZERO;
    transform.translation = Vec3::ZERO;
  }
  

}

