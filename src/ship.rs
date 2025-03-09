use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{
  asset_loader::SceneAssets, collision_detection::{Collider, Player}, game_manager::PlayState, health::Health, hit_marker::HitMarker, hook::{hook_builder, Hook, HookReturnedEvent, Hookable}, input::{InputEventAction, InputEventType, InputMovementEvent, InputTriggerEvent}, movement::{Acceleration, Velocity}, scheduling::GameSchedule, state::GameState, wreck::{Wreck, WreckedEvent}
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(40.0, 0.0, 0.0);
const SHIP_ACCELERATION: f32 = 500.0;
const SHIP_DAMPING: f32 = 150.0;
const SHIP_MAX_SPEED: f32 = 40.0;
const SHIP_MAX_PITCH: f32 = 0.1 * PI;
const SHIP_PITCH_RATE: f32 = 2.;
const SHIP_COLLISION_RADIUS: f32 = 1.8;
const SHIP_COLLISION_DAMAGE: f32 = -1000.0;
const SHIP_INITIAL_HEALTH: f32 = 30.0;

const SHIP_INVINCIBLE_TIME: f32 = 1.5;
const SHIP_INVINCIBLE_FLICKER_RATE: f32 = 15.0;
const SHIP_INVINCIBLE_FLICKER_RATIO: f32 = 3.0;

const CLAW_OFFSET: Vec3 = Vec3::new(0.22188, 0., -0.72352);
const BOUNDS_X_MIN: f32 = -20.;
const BOUNDS_X_MAX: f32 = 50.;
const BOUNDS_Z_MIN: f32 = -30.;
const BOUNDS_Z_MAX: f32 = 30.;
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(PlayState::Alive), spawn_ship)
      .add_systems(
        Update,
        (movement_controls, update_pitch, fire_controls )
          .chain()
          .in_set(GameSchedule::UserInput),
      )
      .add_systems(
        Update,
        (bounds_check, retrieve_hook, check_dead, invincible).in_set(GameSchedule::EntityUpdates),
      )
      .add_systems(
        Update,
        remove_dead_captive.in_set(GameSchedule::PreDespawnEntities),
      );
  }
}

fn spawn_ship(mut commands: Commands, scene_assets: Res<SceneAssets>) {

commands
    .spawn((
      PlayerShip { ..default() },
      SceneRoot(scene_assets.ship.clone()),
      Transform::from_translation(STARTING_TRANSLATION),
      Acceleration {
        acceleration: Vec3::ZERO,
        damping: SHIP_DAMPING,
        max_speed: SHIP_MAX_SPEED,
      },
      Health::new(SHIP_INITIAL_HEALTH),
      Collider {
        radius: SHIP_COLLISION_RADIUS, 
        collision_damage:SHIP_COLLISION_DAMAGE,
      },
      Player,
      Invincible{
        time:Timer::from_seconds(SHIP_INVINCIBLE_TIME, TimerMode::Once),
      },

    ))
    .with_child((
      DisplayHook,
      SceneRoot(scene_assets.hook.clone()),
      Transform::from_translation(CLAW_OFFSET),
    ));
}

#[derive(Component, Default)]
#[require(Transform, Acceleration, Player, HitMarker)]
pub struct PlayerShip {
  target_pitch: f32,
  pitch: f32,
  //hook_out: bool,
  hook: Option<Entity>,
  captive: Option<Entity>,
}

#[derive(Component)]
struct DisplayHook;

#[derive(Component)]
pub struct Captured {
  pub captor: Entity,
}

#[derive(Component)]
pub struct Invincible{
  time:Timer,
}


fn invincible(mut commands:Commands, mut query:Query<(&mut Invincible,  &mut Visibility, Entity)>, time:Res<Time>){
  let Ok((mut invincible, mut visibilty, entity)) = query.get_single_mut() else{
    return;
  };
  invincible.time.tick(time.delta());
  if invincible.time.just_finished(){
    commands.entity(entity).remove::<Invincible>();
    *visibilty = Visibility::Visible;
  }
  else{
    
    *visibilty =  match  (invincible.time.elapsed_secs() * SHIP_INVINCIBLE_FLICKER_RATE % SHIP_INVINCIBLE_FLICKER_RATIO).floor()  {
      0.0 => Visibility::Hidden,
      _ => Visibility::Visible,
    }
  }
}

fn update_pitch(mut query: Query<(&mut PlayerShip, &mut Transform )>, time: Res<Time>) {
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

  mut invinciblitiy_query:Query<&mut Invincible>,
  mut ev_trigger_event: EventReader<InputTriggerEvent>,
  mut display_hook_query: Query<(&mut Visibility, &GlobalTransform), With<DisplayHook>>,
  mut hook_query: Query<&mut Hook>,
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
  match ship.hook {
    Some(hook) => {
      let Ok(mut hook_state) = hook_query.get_mut(hook) else {
        return;
      };
      hook_state.returning = true;
    }
    None => {
      match ship.captive {
        Some(captive) => {
          //eat captive?
          commands.entity(captive).despawn_recursive();
          ship.captive = None;
        }
        None => {
          let Ok((mut display_hook_visible, transform)) = display_hook_query.get_single_mut()
          else {
            return;
          };
          *display_hook_visible = Visibility::Hidden;
          ship.hook = Some(
            commands
              .spawn(hook_builder(
                entity,
                transform.translation(),
                velocity.0,
                scene_assets.hook.clone(),
              ))
              .id(),
          );
          
          //remove invincible if present
          if let Ok(mut invincibility) = invinciblitiy_query.get_single_mut() {
            let time = invincibility.time.duration();
            invincibility.time.set_elapsed(time);
          }
          
        }
      }
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
  
  transform.translation.x = transform.translation.x.clamp(BOUNDS_X_MIN, BOUNDS_X_MAX) ;
  transform.translation.z = transform.translation.z.clamp(BOUNDS_Z_MIN, BOUNDS_Z_MAX) ;

}

fn retrieve_hook(
  mut commands: Commands,
  mut ev_hook_returned: EventReader<HookReturnedEvent>,
  mut display_hook_query: Query<&mut Visibility, With<DisplayHook>>,
  mut ship_query: Query<(&mut PlayerShip, Entity)>,
  mut target_query: Query<(&mut Transform, &mut Hookable)>,
) {
  for &HookReturnedEvent { target } in ev_hook_returned.read() {
    let Ok(mut visible) = display_hook_query.get_single_mut() else {
      return;
    };
    let Ok((mut ship, ship_entity)) = ship_query.get_single_mut() else {
      return;
    };
    ship.hook = None;
    *visible = Visibility::Visible;

    info!("ship hook returned, captive: {:?}", target);

    if target.is_some() {
      let target_entity = target.unwrap();
      let Ok((mut transform, mut hookable)) = target_query.get_mut(target_entity) else {
        return;
      };

      ship.captive = Some(target_entity);
      transform.translation += CLAW_OFFSET;
      hookable.translation += CLAW_OFFSET;
      commands.entity(ship_entity).add_child(target_entity);
      commands.entity(target_entity).insert((
        Captured {
          captor: ship_entity,
        },
        Player,
      ));
    }
  }
}

fn remove_dead_captive(
  mut commands: Commands,
  mut query: Query<(Entity, &Captured, &Health), Without<PlayerShip>>,
  mut ship_query: Query<&mut PlayerShip>,
) {
  let Ok((captive_entity, captured, health)) = query.get_single_mut() else {
    return;
  };
  if health.value <= 0. {
    info!("removing dead captive: {:?}", captive_entity);
    commands
      .entity(captured.captor)
      .remove_children(&[captive_entity]);
    let Ok(mut ship) = ship_query.get_mut(captured.captor) else {
      return;
    };
    ship.captive = None;
  }
}



fn check_dead(
  mut commands: Commands,
  query: Query<(Entity, &Health, &GlobalTransform, &Velocity), (With<PlayerShip>, Without<Wreck>)>,
  hook_query: Query<Entity, With<Hook>>,
  mut ev_wreck_writer: EventWriter<WreckedEvent>,
  mut play_state: ResMut<NextState<PlayState>>,
  scene_assets: Res<SceneAssets>,
) {
  for (entity, health, transform, velocity) in query.iter() {
    if health.value <= 0. {
      info!("dead");
      //   ev_splosion_writer.send(SplosionEvent::new(transform.translation(), 3.0,velocity.0));
      ev_wreck_writer.send(WreckedEvent::new(
        scene_assets.ship.clone(),
        transform.translation(),
        transform.rotation(),
        velocity.0,
        1.2,
        3.0,
        3.0,
      ));
      commands.entity(entity).despawn_recursive();
      play_state.set(PlayState::Dead);
      //get rid of any floating hooks
      let Ok(hook_entity) = hook_query.get_single() else{ continue; };
      commands.entity(hook_entity).despawn_recursive();
    }
  }
}