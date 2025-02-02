use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{
  asset_loader::SceneAssets,
  collision_detection::{Collider, Player},
  health::Health,
  hook::{
    Hook, HookReturnedEvent, HOOK_COLLISION_RADIUS, HOOK_DAMPING, HOOK_LAUNCH_SPEED, HOOK_MAX_SPEED,
  },
  movement::{Acceleration, Velocity},
  scheduling::GameSchedule,
  state::GameState,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(40.0, 0.0, 0.0);
const SHIP_ACCELERATION: f32 = 500.0;
const SHIP_DAMPING: f32 = 150.0;
const SHIP_MAX_SPEED: f32 = 40.0;
const SHIP_MAX_PITCH: f32 = 0.1 * PI;
const SHIP_PITCH_RATE: f32 = 2.;
const SHIP_COLLISION_RADIUS: f32 = 1.8;
const SHIP_INITIAL_HEALTH: f32 = 100.0;

const CLAW_OFFSET: Vec3 = Vec3::new(0.22188, 0., -0.72352);
const BOUNDS_X_MIN: f32 = -20.;
const BOUNDS_X_MAX: f32 = 50.;
const BOUNDS_Z_MIN: f32 = -30.;
const BOUNDS_Z_MAX: f32 = 30.;
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Playing), spawn_ship)
      .add_systems(
        Update,
        (movement_controls, update_pitch, fire_controls)
          .chain()
          .in_set(GameSchedule::UserInput),
      )
      .add_systems(Update, bounds_check.in_set(GameSchedule::BoundsCheck))
      .add_systems(Update, retrieve_hook.in_set(GameSchedule::DespawnEntities));
  }
}

fn spawn_ship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
  commands
    .spawn((
      PlayerShip {
        target_pitch: 0.,
        pitch: 0.,
        hook_out: false,
      },
      SceneRoot(scene_assets.ship.clone()),
      Transform::from_translation(STARTING_TRANSLATION),
      Acceleration {
        acceleration: Vec3::ZERO,
        damping: SHIP_DAMPING,
        max_speed: SHIP_MAX_SPEED,
      },
      Health(SHIP_INITIAL_HEALTH),
      Collider {
        radius: SHIP_COLLISION_RADIUS,
      },
      Player,
    ))
    .with_child((
      DisplayHook,
      SceneRoot(scene_assets.hook.clone()),
      Transform::from_translation(CLAW_OFFSET),
    ));
}

#[derive(Component, Default)]
#[require(Transform, Acceleration, Player)]
pub struct PlayerShip {
  target_pitch: f32,
  pitch: f32,
  hook_out: bool,
}

#[derive(Component)]
struct DisplayHook;

fn update_pitch(mut query: Query<(&mut PlayerShip, &mut Transform)>, time: Res<Time>) {
  let Ok((mut ship, mut transform)) = query.get_single_mut() else {
    return;
  };
  let diff = ship.target_pitch - ship.pitch;
  let max_turn = SHIP_PITCH_RATE * time.delta_secs();
  if max_turn > diff.abs() {
    ship.pitch = ship.target_pitch;
  } else {
    ship.pitch += diff.signum() * max_turn;
  }
  transform.rotation = Quat::from_rotation_y(ship.pitch);
}

fn fire_controls(
  mut commands: Commands,
  mut query: Query<(Entity, &mut PlayerShip, &Velocity)>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut display_hook_query: Query<(&mut Visibility, &GlobalTransform), With<DisplayHook>>,
  scene_assets: Res<SceneAssets>,
) {
  let Ok((entity, mut ship, velocity)) = query.get_single_mut() else {
    return;
  };
  if keyboard_input.pressed(KeyCode::Space) && !ship.hook_out {
    ship.hook_out = true;
    let Ok((mut display_hook_visible, transform)) = display_hook_query.get_single_mut() else {
      return;
    };

    *display_hook_visible = Visibility::Hidden;
    //spawn hook
    commands.spawn((
      Hook::new(entity),
      Player,
      SceneRoot(scene_assets.hook.clone()),
      Velocity(velocity.0 + Vec3::new(-HOOK_LAUNCH_SPEED, 0., 0.)), //(transform.right() * -60.0)),
      Acceleration {
        acceleration: Vec3::ZERO,
        damping: HOOK_DAMPING,
        max_speed: HOOK_MAX_SPEED,
      },
      Transform::from_translation(transform.translation()),
      Collider::new(HOOK_COLLISION_RADIUS),
    ));
  }
}

fn movement_controls(
  mut query: Query<(&mut Acceleration, &mut PlayerShip)>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
) {
  let Ok((mut acceleration, mut ship)) = query.get_single_mut() else {
    return;
  };
  let mut acc = Vec3::ZERO;

  if keyboard_input.pressed(KeyCode::KeyD) {
    acc.x -= 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyA) {
    acc.x += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyW) {
    acc.z += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyS) {
    acc.z -= 1.;
  }
  acc = acc.normalize_or_zero();
  acceleration.acceleration = acc * SHIP_ACCELERATION;
  ship.target_pitch = acc.z * SHIP_MAX_PITCH;
}

fn bounds_check(mut query: Query<&mut Transform, With<PlayerShip>>) {
  let Ok(mut transform) = query.get_single_mut() else {
    return;
  };
  let translation = transform.translation;
  if translation.x > BOUNDS_X_MAX {
    transform.translation.x = BOUNDS_X_MAX;
  }
  if translation.x < BOUNDS_X_MIN {
    transform.translation.x = BOUNDS_X_MIN;
  }
  if translation.z > BOUNDS_Z_MAX {
    transform.translation.z = BOUNDS_Z_MAX;
  }
  if translation.z < BOUNDS_Z_MIN {
    transform.translation.z = BOUNDS_Z_MIN;
  }
}

fn retrieve_hook(
  mut ev_hook_returned: EventReader<HookReturnedEvent>,
  mut display_hook_query: Query<&mut Visibility, With<DisplayHook>>,
  mut ship_query: Query<&mut PlayerShip>,
) {
  for &HookReturnedEvent { target } in ev_hook_returned.read() {
    let Ok(mut visible) = display_hook_query.get_single_mut() else {
      return;
    };

    *visible = Visibility::Visible;

    let Ok(mut ship) = ship_query.get_single_mut() else {
      return;
    };
    ship.hook_out = false;
    //trap target or something
  }
}
