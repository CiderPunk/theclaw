use bevy::{prelude::*, window::PrimaryWindow};

use crate::scheduling::GameSchedule;

#[derive(PartialEq)]
pub enum InputEventType {
  Pressed,
  Released,
}

#[derive(PartialEq)]
pub enum InputEventAction {
  Shoot,
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<InputMovementEvent>()
      .add_event::<InputTriggerEvent>()
      .add_systems(Startup, init_input_resources)
      .add_systems(
        Update,
        (read_keys, read_mouse, read_touch, read_gamepads).in_set(GameSchedule::UserInput),
      );
  }
}

#[derive(Event)]
pub struct InputMovementEvent {
  pub direction: Vec2,
}

impl InputMovementEvent {
  pub fn new(direction: Vec2) -> Self {
    Self { direction }
  }
}

#[derive(Event)]
pub struct InputTriggerEvent {
  pub action: InputEventAction,
  pub input_type: InputEventType,
}

impl InputTriggerEvent {
  pub fn new(action: InputEventAction, input_type: InputEventType) -> Self {
    Self { action, input_type }
  }
}

#[derive(Resource)]
struct MouseResource {
  last: Vec2,
}

#[derive(Resource)]
struct TouchResource {
  last: Vec2,
}

fn init_input_resources(mut commands: Commands) {
  commands.insert_resource(MouseResource { last: Vec2::ZERO });
  commands.insert_resource(TouchResource { last: Vec2::ZERO });
}

fn read_gamepads(
  gamepads: Query<&Gamepad>,
  mut ev_movement_event: EventWriter<InputMovementEvent>,
  mut ev_trigger_event: EventWriter<InputTriggerEvent>,
) {
  for gamepad in &gamepads {
    if gamepad.just_pressed(GamepadButton::South) {
      ev_trigger_event.send(InputTriggerEvent::new(
        InputEventAction::Shoot,
        InputEventType::Pressed,
      ));
    } else if gamepad.just_released(GamepadButton::South) {
      ev_trigger_event.send(InputTriggerEvent::new(
        InputEventAction::Shoot,
        InputEventType::Released,
      ));
    }

    let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
    let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
    let dir: Vec2 = Vec2::new(-left_stick_x, left_stick_y);

    if dir.length_squared() > 0.1 {
      ev_movement_event.send(InputMovementEvent::new(dir));
    }
  }
}

fn read_touch(
  touches: Res<Touches>,
  mut ev_movement_event: EventWriter<InputMovementEvent>,
  mut ev_trigger_event: EventWriter<InputTriggerEvent>,
  mut touch_location: ResMut<TouchResource>,
) {


  for touch in touches.iter_just_pressed() {

    info!("touch down: {:?}", touch.id());
    if touch.id() == 0 {
      touch_location.last = touch.position();
    } else if touch.id() == 1 {
      ev_trigger_event.send(InputTriggerEvent::new(
        InputEventAction::Shoot,
        InputEventType::Pressed,
      ));
    }
  }
  for touch in touches.iter_just_released() {
    info!("touch up: {:?}", touch.id());
    if touch.id() == 1 {
      ev_trigger_event.send(InputTriggerEvent::new(
        InputEventAction::Shoot,
        InputEventType::Released,
      ));
    }
  }

  for touch in touches.iter() {
    if touch.id() == 0 {
      let diff = touch_location.last - touch.position();
      if diff.length_squared() > 0.5 {
        ev_movement_event.send(InputMovementEvent::new(diff * 2.));
      }
      touch_location.last = touch.position();
    }
  }
}

fn read_mouse(
  buttons: Res<ButtonInput<MouseButton>>,
  window: Single<&Window, With<PrimaryWindow>>,
  mut ev_movement_event: EventWriter<InputMovementEvent>,
  mut ev_trigger_event: EventWriter<InputTriggerEvent>,
  mut mouse_location: ResMut<MouseResource>,
) {
  if buttons.just_pressed(MouseButton::Right) {
    ev_trigger_event.send(InputTriggerEvent::new(
      InputEventAction::Shoot,
      InputEventType::Pressed,
    ));
  }
  if buttons.just_released(MouseButton::Right) {
    ev_trigger_event.send(InputTriggerEvent::new(
      InputEventAction::Shoot,
      InputEventType::Released,
    ));
  }

  if buttons.pressed(MouseButton::Left) {
    if let Some(pos) = window.cursor_position() {
      if buttons.just_pressed(MouseButton::Left) {
        mouse_location.last = pos;
      } else {
        let diff = mouse_location.last - pos;
        if diff.length_squared() > 0.5 {
          ev_movement_event.send(InputMovementEvent::new(diff * 2.));
        }
        mouse_location.last = pos;
      }
    }
  }
}

fn read_keys(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut ev_movement_event: EventWriter<InputMovementEvent>,
  mut ev_trigger_event: EventWriter<InputTriggerEvent>,
) {
  let mut dir: Vec2 = Vec2::ZERO;
  if keyboard_input.pressed(KeyCode::KeyD) {
    dir.x -= 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyA) {
    dir.x += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyW) {
    dir.y += 1.;
  }
  if keyboard_input.pressed(KeyCode::KeyS) {
    dir.y -= 1.;
  }
  if dir != Vec2::ZERO {
    ev_movement_event.send(InputMovementEvent::new(dir));
  }

  if keyboard_input.just_pressed(KeyCode::Space) {
    ev_trigger_event.send(InputTriggerEvent::new(
      InputEventAction::Shoot,
      InputEventType::Pressed,
    ));
  }

  if keyboard_input.just_released(KeyCode::Space) {
    ev_trigger_event.send(InputTriggerEvent::new(
      InputEventAction::Shoot,
      InputEventType::Released,
    ));
  }
}
