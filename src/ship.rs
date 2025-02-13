use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{
  asset_loader::SceneAssets,
  collision_detection::{Collider, Player},
  health::Health,
  hook::{hook_builder, Hook, HookControlEvent, HookControlEventType, HookReturnedEvent},
  input::{InputEventAction, InputEventType, InputMovementEvent, InputTriggerEvent},
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
        ..default()
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
  //hook_out: bool,
  hook:Option<Entity>,
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
  mut commands:Commands,
  mut query: Query<(Entity, &mut PlayerShip, &Velocity)>,
  mut ev_trigger_event: EventReader<InputTriggerEvent>,
  mut display_hook_query: Query<(&mut Visibility, &GlobalTransform), With<DisplayHook>>,
  scene_assets: Res<SceneAssets>,
) {
  let Ok((entity, mut ship, velocity)) = query.get_single_mut() else {
    return;
  };

  let mut shoot = false;
  for InputTriggerEvent { action, input_type } in ev_trigger_event.read() {
    if *action == InputEventAction::Shoot && *input_type == InputEventType::Pressed {
      shoot = true;
    }
  }
  if !shoot {
    return;
  }
  match ship.hook{
    Some(hook) =>{


    }
    None =>{
      let Ok((mut display_hook_visible, transform)) = display_hook_query.get_single_mut() else {
        return;
      };
      *display_hook_visible = Visibility::Hidden;
      ship.hook = Some(commands.spawn(
        hook_builder(entity, transform.translation(), velocity.0, scene_assets.hook.clone())
      ).id());
    }
  }
}

fn movement_controls(
  mut query: Query<(&mut Acceleration, &mut PlayerShip)>,
  mut ev_movement_event: EventReader<InputMovementEvent>,
  //keyboard_input: Res<ButtonInput<KeyCode>>,
) {
  let Ok((mut acceleration, mut ship)) = query.get_single_mut() else {
    return;
  };
  let mut acc = Vec2::ZERO;
  for InputMovementEvent { direction } in ev_movement_event.read() {
    acc += direction;
  }

  acc = acc.normalize_or_zero();
  acceleration.acceleration = Vec3::new(acc.x, 0., acc.y) * SHIP_ACCELERATION;
  ship.target_pitch = acc.y * SHIP_MAX_PITCH;
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
    ship.hook = None;
    //trap target or something
  }
}
